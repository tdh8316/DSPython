extern crate either;

use std::borrow::Borrow;
use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::values::{AnyValueEnum, BasicValue, BasicValueEnum, FunctionValue, PointerValue};
use inkwell::AddressSpace;
use num_bigint::{BigInt, BigUint};
use num_traits::{Signed, ToPrimitive};
use rustpython_compiler::symboltable::SymbolTable;
use rustpython_parser::ast;
use rustpython_parser::ast::{Expression, Operator, Parameters, Suite};

use crate::codegen::value::*;

use self::either::Either;
use std::any::Any;

#[derive(Debug, Clone, Copy)]
struct CompileContext {
    in_loop: bool,
    func: bool,
}

#[derive(Debug)]
pub struct Compiler<'a, 'ctx> {
    symbol_table_stack: Vec<SymbolTable>,
    source_path: String,
    current_source_location: ast::Location,
    ctx: CompileContext,

    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,

    variables: HashMap<String, (ValueType, PointerValue<'ctx>)>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}
fn truncate_bigint_to_u32(a: &BigInt) -> u32 {
    fn truncate_biguint_to_u32(a: &BigUint) -> u32 {
        use std::u32;
        let mask = BigUint::from(u32::MAX);
        (a & mask.borrow()).to_u32().unwrap()
    }
    let was_negative = a.is_negative();
    let abs = a.abs().to_biguint().unwrap();
    let truncated = truncate_biguint_to_u32(&abs);
    if was_negative {
        truncated.wrapping_neg()
    } else {
        truncated
    }
}
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        source_path: String,
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        pass_manager: &'a PassManager<FunctionValue<'ctx>>,
        module: &'a Module<'ctx>,
    ) -> Self {
        {
            Compiler {
                symbol_table_stack: Vec::new(),
                source_path,
                current_source_location: ast::Location::default(),
                ctx: CompileContext {
                    in_loop: false,
                    func: false,
                },
                context,
                builder,
                fpm: pass_manager,
                module,
                variables: HashMap::new(),
                fn_value_opt: None,
            }
        }
    }

    #[inline]
    fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        {
            self.module.get_function(name)
        }
    }

    fn set_source_location(&mut self, location: ast::Location) {
        self.current_source_location = location;
    }

    fn compile_stmt_function_def(
        &mut self,
        name: &String,
        args: &Box<Parameters>,
        body: &Suite,
        returns: &Option<Expression>,
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
        self.ctx.func = false;
    }

    fn compile_stmt(&mut self, stmt: &ast::Statement) {
        self.set_source_location(stmt.location);

        use rustpython_parser::ast::StatementType::*;
        match &stmt.node {
            Expression { expression } => {
                self.compile_expr(expression);
            }
            FunctionDef {
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
                self.compile_stmt_function_def(name, args, body, returns);
            }
            Assign { targets, value } => {
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
            Return { value } => {
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
            ImportFrom {
                level,
                module,
                names,
            } => {
                let target = module.as_ref().expect("Unknown module name");
                if target.clone() == String::from("uno") {
                    // Do nothing
                } else {
                    panic!("Import is not implemented");
                }
            }
            Pass => {}
            _ => panic!(
                "{:?}\nNotImplemented statement {:?}",
                self.current_source_location, stmt.node
            ),
        }
    }

    fn compile_op(&mut self, a: &Expression, op: &Operator, b: &Expression) -> Value<'ctx> {
        let l = self.compile_expr(a);
        let r = self.compile_expr(b);

        l.invoke_handler(
            ValueHandler::new()
                .handle_int(&|_, lhs_value| {
                    r.invoke_handler(ValueHandler::new().handle_int(&|_, rhs_value| Value::I16 {
                        value: match op {
                            ast::Operator::Add {} => {
                                self.builder.build_int_add(lhs_value, rhs_value, "add")
                            }
                            ast::Operator::Sub {} => {
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
                    r.invoke_handler(
                        ValueHandler::new().handle_float(&|_, rhs_value| Value::F32 {
                            value: match op {
                                ast::Operator::Add {} => {
                                    self.builder.build_float_add(lhs_value, rhs_value, "add")
                                }
                                ast::Operator::Sub {} => {
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

        use rustpython_parser::ast::ExpressionType::*;
        match &expr.node {
            Number { value } => match value {
                ast::Number::Integer { value } => {
                    let n = truncate_bigint_to_u32(value) as u64;
                    Value::I16 {
                        value: self
                            .context
                            .i16_type() // TODO: Match type
                            .const_int(n, false),
                    }
                }
                ast::Number::Float { value } => Value::F32 {
                    value: self.context.f32_type().const_float(value.clone()),
                },
                _ => panic!(
                    "{:?}\nUnsupported number value",
                    self.current_source_location
                ),
            },
            Call {
                function,
                args,
                keywords,
            } => self.compile_expr_call(function, args),
            Binop { a, op, b } => self.compile_op(a, op, b),
            Identifier { name } => {
                let refer = self.variables.get(name).expect(
                    format!(
                        "{:?}\nUndefined variable {}",
                        self.current_source_location, name
                    )
                    .as_str(),
                );
                match refer.1 {
                    var => {
                        Value::from_basic_value(refer.0, self.builder.build_load(var, name).into())
                    }
                }
            }
            None => Value::Void,
            _ => panic!(
                "{:?}\nNotImplemented expression {:?}",
                self.current_source_location, expr.node
            ),
        }
    }

    fn compile_expr_call(&mut self, func: &Box<Expression>, args: &Vec<Expression>) -> Value<'ctx> {
        // TODO: Args
        let mut func_name = match &func.node {
            ast::ExpressionType::Identifier { name } => name,
            _ => {
                panic!("{:?}\nUnknown function name", self.current_source_location);
            }
        }
        .as_str();

        func_name = match func_name {
            "pin_mode" => "pinMode",
            "digital_write" => "digitalWrite",
            _ => func_name,
        };

        let func = self.get_function(func_name).expect(
            format!(
                "{:?}\nFunction '{}' is not defined",
                self.current_source_location, func_name
            )
            .as_str(),
        );

        let args_proto = func.get_params();

        let mut args_value: Vec<BasicValueEnum> = vec![];

        for (expr, proto) in args.iter().zip(args_proto.iter()) {
            // TODO: Currently only convert to i8. Match arguments' types
            let value = match self.compile_expr(expr) {
                Value::I16 { value } => value,
                _ => panic!("NotImplemented function call argument"),
            };
            /* self.builder.build_int_truncate(
                self.context.i8_type().const_int(vv, false),
                self.context.i8_type(),
                "cast -> i8",
            );*/
            let cast = self
                .builder
                .build_int_truncate(value, proto.get_type().into_int_type(), "i8");
            args_value.push(BasicValueEnum::IntValue(cast))
        }

        let res = self.builder.build_call(func, args_value.as_slice(), "call");
        res.set_tail_call(true);

        match res.try_as_basic_value() {
            Either::Left(bv) => Value::from_basic_value(ValueType::Void, bv),
            Either::Right(_) => Value::Void,
        }
    }

    pub fn generate_prototypes(&mut self) {
        self.module.add_function(
            "pinMode",
            self.context.void_type().fn_type(
                &[self.context.i8_type().into(), self.context.i8_type().into()],
                false,
            ),
            None,
        );
        self.module.add_function(
            "delay",
            self.context.void_type().fn_type(
                &[self.context.i32_type().into()],
                false,
            ),
            None,
        );
        self.module.add_function(
            "digitalWrite",
            self.context.void_type().fn_type(
                &[self.context.i8_type().into(), self.context.i8_type().into()],
                false,
            ),
            None,
        );
    }

    pub fn compile_program(&mut self, program: ast::Program) {
        self.generate_prototypes();
        for statement in program.statements.iter() {
            if let rustpython_parser::ast::StatementType::Expression { ref expression } =
                statement.node
            {
                self.compile_expr(&expression);
            } else {
                self.compile_stmt(&statement);
            }
        }
    }
}
