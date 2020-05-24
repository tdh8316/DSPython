use rustpython_parser::ast;

use crate::compiler::Compiler;
use crate::irgen::expr::CGExpr;
use crate::value::{Value, ValueHandler, ValueType};
use inkwell::{IntPredicate, FloatPredicate};
use inkwell::values::IntValue;

pub trait CGStmt<'a, 'ctx> {
    fn compile_stmt(&mut self, stmt: &ast::Statement);
    fn compile_stmt_function_def(
        &mut self,
        name: &String,
        _args: &Box<ast::Parameters>,
        body: &ast::Suite,
        _returns: &Option<ast::Expression>,
    );
    fn compile_stmt_conditional(
        &mut self,
        cond: Value<'ctx>,
        body: &Vec<ast::Statement>,
        orelse: Option<&Vec<ast::Statement>>,
    );
}

impl<'a, 'ctx> CGStmt<'a, 'ctx> for Compiler<'a, 'ctx> {
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
            StatementType::If { test, body, orelse } => {
                let cond = self.compile_expr(test);

                match orelse {
                    None /*Only if:*/ => {
                        self.compile_stmt_conditional(cond, body, None);
                    }
                    Some(statements) => {
                        self.compile_stmt_conditional(cond, body, Some(statements));
                    }
                }
            }
            StatementType::Pass => {}
            _ => panic!(
                "{:?}\nNotImplemented statement {:?}",
                self.current_source_location, stmt.node
            ),
        }
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
        self.fn_value_opt = Some(f);

        for statement in body.iter() {
            self.compile_stmt(statement);
        }

        // self.compile_expr(returns.as_ref().unwrap());

        self.ctx.func = false;
    }

    fn compile_stmt_conditional(
        &mut self,
        cond: Value<'ctx>,
        body: &Vec<ast::Statement>,
        orelse: Option<&Vec<ast::Statement>>,
    ) {
        // TODO: Accept other cond type
        /*if cond.get_type() != ValueType::Bool {
            panic!(
                "Expected {:?}, but got {:?} type.",
                ValueType::Bool,
                cond.get_type()
            );
        }*/

        let parent = self.fn_value();

        let then_bb = self.context.append_basic_block(parent, "then");
        let else_bb = self.context.append_basic_block(parent, "else");
        let cont_bb = self.context.append_basic_block(parent, "cont");

        let cond = cond.invoke_handler(
            ValueHandler::new()
                .handle_bool(&|_, value| value)

                // In Python, all integers except 0 are considered true.
                .handle_int(&|value, _| {
                    let a = value;
                    let b = Value::I16 { value: self.context.i16_type().const_zero() };

                    // This LLVM expression is same as `lhs_value != 0`
                    // Therefore all integers except 0 are considered true.
                    let c = a.invoke_handler(
                        ValueHandler::new()
                            .handle_int(&|_, lhs_value| {
                                b.invoke_handler(ValueHandler::new().handle_int(&|_, rhs_value| {
                                    Value::Bool {
                                        value: self.builder.build_int_compare(
                                            IntPredicate::NE,
                                            lhs_value,
                                            rhs_value,
                                            "a",
                                        ),
                                    }
                                }))
                            })
                            .handle_float(&|_, lhs_value| {
                                b.invoke_handler(ValueHandler::new().handle_float(&|_, rhs_value| {
                                    Value::Bool {
                                        value: self.builder.build_float_compare(
                                            FloatPredicate::ONE,
                                            lhs_value,
                                            rhs_value,
                                            "a",
                                        ),
                                    }
                                }))
                            }),
                    );

                    c.invoke_handler(
                        ValueHandler::new().handle_bool(&|_, value| value)
                    )
                }),
        );

        self.builder
            .build_conditional_branch(cond, then_bb, else_bb);

        self.builder.position_at_end(then_bb);
        for statement in body.iter() {
            self.compile_stmt(statement);
        }
        self.builder.build_unconditional_branch(cont_bb);

        let _then_bb = self.builder.get_insert_block().unwrap();

        self.builder.position_at_end(else_bb);

        match orelse {
            Some(statements) => {
                for statement in statements.iter() {
                    self.compile_stmt(statement);
                }
            }
            None => {}
        }

        self.builder.build_unconditional_branch(cont_bb);

        let _else_bb = self.builder.get_insert_block().unwrap();

        self.builder.position_at_end(cont_bb);
    }
}
