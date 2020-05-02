use std::collections::HashMap;

use either::Either;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use rustpython_parser::ast;

use crate::compiler::mangle::mangling;
use crate::compiler::prototypes::generate_prototypes;
use crate::value::convert::{truncate_bigint_to_u64, try_get_constant_string};
use crate::value::value::{Value, ValueHandler, ValueType};

#[derive(Debug, Clone, Copy)]
struct CompileContext {
    in_loop: bool,
    func: bool,
}

#[derive(Debug)]
pub struct Compiler<'a, 'ctx> {
    // symbol_table_stack: Vec<SymbolTable>,
    source_path: String,
    current_source_location: ast::Location,
    ctx: CompileContext,

    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub pm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,

    variables: HashMap<String, (ValueType, PointerValue<'ctx>)>,
    // fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        source_path: String,
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        pass_manager: &'a PassManager<FunctionValue<'ctx>>,
        module: &'a Module<'ctx>,
    ) -> Self {
        Compiler {
            // symbol_table_stack: Vec::new(),
            source_path,
            current_source_location: ast::Location::default(),
            ctx: CompileContext {
                in_loop: false,
                func: false,
            },
            context,
            builder,
            pm: pass_manager,
            module,
            variables: HashMap::new(),
            // fn_value_opt: None,
        }
    }

    fn set_source_location(&mut self, location: ast::Location) {
        self.current_source_location = location;
    }

    #[inline]
    fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(name)
    }

    fn compile_stmt_function_def(
        &mut self,
        name: &String,
        _args: &Box<ast::Parameters>,
        body: &ast::Suite,
        _returns: &Option<ast::Expression>,
    ) {
        // TODO: args, implicit returns
        let f = self
            .module
            .add_function(name, self.context.void_type().fn_type(&[], false), None);
        let bb = self.context.append_basic_block(f, "");

        self.builder.position_at_end(bb);

        self.ctx.func = true;

        for statement in body.iter() {
            self.compile_stmt(statement);
        }

        // self.compile_expr(returns.as_ref().unwrap());

        self.ctx.func = false;
    }

    fn compile_stmt(&mut self, stmt: &ast::Statement) {
        self.set_source_location(stmt.location);

        use rustpython_parser::ast::StatementType;
        match &stmt.node {
            StatementType::Expression { expression } => {
                self.compile_expr(expression);
            }
            StatementType::FunctionDef {
                is_async,
                name,
                args,
                body,
                decorator_list,
                returns,
            } => {
                if *is_async {
                    panic!("{:?}\nAsync is not supported", self.current_source_location)
                }
                let _decorators = decorator_list;
                self.compile_stmt_function_def(name, args, body, returns);
            }
            StatementType::Assign { targets, value } => {
                let target = match &targets.last().unwrap().node {
                    ast::ExpressionType::Identifier { name } => name,
                    _ => panic!(
                        "{:?}\nUnsupported assign target",
                        self.current_source_location
                    ),
                };
                let value = self.compile_expr(value);
                let ty = value.get_type();

                if self.ctx.func {
                    let p = self
                        .builder
                        .build_alloca(ty.to_basic_type(self.context), target);
                    self.builder.build_store(p, value.to_basic_value());
                    self.variables.insert(target.to_string(), (ty, p));
                } else {
                    let p = self
                        .module
                        .add_global(ty.to_basic_type(self.context), None, target);
                    p.set_unnamed_addr(true);
                    p.set_initializer(&value.to_basic_value());
                    self.variables
                        .insert(target.to_string(), (ty, p.as_pointer_value()));
                }
            }
            StatementType::Return { value } => {
                if !self.ctx.func {
                    panic!(
                        "{:?}\n'return' outside function",
                        self.current_source_location
                    )
                }
                let value = self.compile_expr(value.as_ref().expect("No return value"));
                self.builder.build_return(match value {
                    Value::Void => None,
                    _ => panic!(
                        "{:?}\nUnsupported return type {:?}",
                        self.current_source_location,
                        value.get_type()
                    ),
                });
            }
            StatementType::ImportFrom {
                level,
                module,
                names,
            } => {
                let _level = level;
                let target = module.as_ref().expect("Unknown module name");
                let _names = names;
                if target.clone() == String::from("uno") {
                    // Do nothing
                } else {
                    panic!("Import is not implemented");
                }
            }
            StatementType::Pass => {}
            _ => panic!(
                "{:?}\nNotImplemented statement {:?}",
                self.current_source_location, stmt.node
            ),
        }
    }

    fn compile_expr_call(
        &mut self,
        func: &Box<ast::Expression>,
        args: &Vec<ast::Expression>,
    ) -> Value<'ctx> {
        let mut func_name = match &func.node {
            ast::ExpressionType::Identifier { name } => name,
            _ => {
                panic!(
                    "{:?}\nUnknown function name {:?}",
                    self.current_source_location, func.node
                );
            }
        }
        .to_string();

        let func = match self.get_function(func_name.as_ref()) {
            Some(f) => f,
            None => {
                let at = self.compile_expr(args.clone().first().unwrap()).get_type();
                let func_name = mangling(&mut func_name, at);
                self.get_function(func_name).expect(
                    format!(
                        "{:?}\nFunction '{}' is not defined",
                        self.current_source_location, func_name
                    )
                    .as_str(),
                )
            }
        };

        let args_proto = func.get_params();

        let mut args_value: Vec<BasicValueEnum> = vec![];

        for (expr, proto) in args.iter().zip(args_proto.iter()) {
            let value = self.compile_expr(expr);
            match value {
                Value::I16 { value } => {
                    let cast = self.builder.build_int_truncate(
                        value,
                        proto.get_type().into_int_type(),
                        "icast",
                    );
                    args_value.push(BasicValueEnum::IntValue(cast))
                }
                Value::Str { value } => args_value.push(BasicValueEnum::PointerValue(value)),
                _ => panic!(
                    "{:?}\nNotImplemented argument type",
                    self.current_source_location
                ),
            }
        }

        let res = self.builder.build_call(func, args_value.as_slice(), "call");
        res.set_tail_call(true);

        match res.try_as_basic_value() {
            Either::Left(bv) => Value::from_basic_value(ValueType::Void, bv),
            Either::Right(_) => Value::Void,
        }
    }

    fn compile_op(&mut self, a: Value<'ctx>, op: &ast::Operator, b: Value<'ctx>) -> Value<'ctx> {
        use rustpython_parser::ast::Operator;
        a.invoke_handler(
            ValueHandler::new()
                .handle_int(&|_, lhs_value| {
                    b.invoke_handler(ValueHandler::new().handle_int(&|_, rhs_value| Value::I16 {
                        value: match op {
                            Operator::Add {} => {
                                self.builder.build_int_add(lhs_value, rhs_value, "add")
                            }
                            Operator::Sub {} => {
                                self.builder.build_int_sub(lhs_value, rhs_value, "sub")
                            }
                            _ => panic!(
                                "{:?}\nNotImplemented operator for i16",
                                self.current_source_location
                            ),
                        },
                    }))
                })
                .handle_float(&|_, lhs_value| {
                    b.invoke_handler(
                        ValueHandler::new().handle_float(&|_, rhs_value| Value::F32 {
                            value: match op {
                                Operator::Add {} => {
                                    self.builder.build_float_add(lhs_value, rhs_value, "add")
                                }
                                Operator::Sub {} => {
                                    self.builder.build_float_sub(lhs_value, rhs_value, "sub")
                                }
                                _ => panic!(
                                    "{:?}\nNotImplemented operator for f32",
                                    self.current_source_location
                                ),
                            },
                        }),
                    )
                }),
        )
    }

    fn compile_expr(&mut self, expr: &ast::Expression) -> Value<'ctx> {
        self.set_source_location(expr.location);

        use rustpython_parser::ast::ExpressionType;
        match &expr.node {
            ExpressionType::Number { value } => match value {
                ast::Number::Integer { value } => Value::I16 {
                    value: self
                        .context
                        .i16_type()
                        .const_int(truncate_bigint_to_u64(value), true),
                },
                ast::Number::Float { value } => Value::F32 {
                    value: self.context.f32_type().const_float(*value),
                },
                ast::Number::Complex { real: _, imag: _ } => {
                    panic!(
                        "{:?}\nNotImplemented  builder for imaginary number",
                        self.current_source_location
                    );
                }
            },
            ExpressionType::String { value } => {
                let v = try_get_constant_string(value).unwrap();
                if self.ctx.func {
                    Value::Str {
                        value: self
                            .builder
                            .build_global_string_ptr(v.as_str(), ".str")
                            .as_pointer_value(),
                    }
                } else {
                    panic!("NotImplemented builder for global string")
                }
            }
            ExpressionType::Call {
                function,
                args,
                keywords,
            } => {
                let _keywords = keywords;
                self.compile_expr_call(function, args)
            }
            ExpressionType::Binop { a, op, b } => {
                let a = self.compile_expr(a);
                let b = self.compile_expr(b);
                self.compile_op(a, op, b)
            }
            ExpressionType::Identifier { name } => {
                let (ty, pointer) = self.variables.get(name).expect(
                    format!(
                        "{:?}\nName '{}' is not defined",
                        self.current_source_location, name
                    )
                    .as_str(),
                );
                match *pointer {
                    var => Value::from_basic_value(*ty, self.builder.build_load(var, name).into()),
                }
            }
            ExpressionType::None => Value::Void,
            _ => {
                panic!(
                    "{:?}\nNotImplemented expression",
                    self.current_source_location
                );
            }
        }
    }

    pub fn compile(&mut self, program: ast::Program) {
        generate_prototypes(&self.module, &self.context);

        for statement in program.statements.iter() {
            if let ast::StatementType::Expression { ref expression } = statement.node {
                self.compile_expr(&expression);
            } else {
                self.compile_stmt(&statement);
            }
        }
    }
}
