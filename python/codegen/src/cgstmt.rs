use std::option::Option::Some;

use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValue;
use inkwell::{FloatPredicate, IntPredicate};

use dsp_compiler_error::*;
use dsp_compiler_value::value::{Value, ValueHandler, ValueType};
use dsp_python_parser::ast;

use crate::cgexpr::CGExpr;
use crate::scope::LLVMVariableAccessor;
use crate::CodeGen;

pub trait CGStmt<'a, 'ctx> {
    fn compile_stmt(&mut self, stmt: &ast::Statement) -> Result<(), LLVMCompileError>;
    fn compile_stmt_function_def(
        &mut self,
        name: &String,
        args: &Box<ast::Parameters>,
        body: &ast::Suite,
        returns: &Option<ast::Expression>,
    ) -> Result<(), LLVMCompileError>;
    fn compile_stmt_conditional(
        &mut self,
        cond: Value<'ctx>,
        body: &Vec<ast::Statement>,
        orelse: Option<&Vec<ast::Statement>>,
    ) -> Result<(), LLVMCompileError>;
    fn compile_stmt_while(
        &mut self,
        test: &ast::Expression,
        body: &ast::Suite,
        orelse: &Option<ast::Suite>,
    ) -> Result<(), LLVMCompileError>;
}

impl<'a, 'ctx> CGStmt<'a, 'ctx> for CodeGen<'a, 'ctx> {
    fn compile_stmt(&mut self, stmt: &ast::Statement) -> Result<(), LLVMCompileError> {
        self.set_source_location(stmt.location);
        use dsp_python_parser::ast::StatementType;
        match &stmt.node {
            StatementType::Assign { targets, value } => {
                if targets.len() > 1 {
                    return Err(LLVMCompileError::new(
                        self.get_source_location(),
                        LLVMCompileErrorType::NotImplemented(Some(
                            "Variable unpacking is not implemented.".to_string(),
                        )),
                    ));
                }
                let target = targets.last().unwrap();
                let name = match &target.node {
                    ast::ExpressionType::Identifier { name } => name,
                    _ => {
                        return Err(LLVMCompileError::new(
                            self.get_source_location(),
                            LLVMCompileErrorType::NotImplemented(None),
                        ))
                    }
                };
                let value = self.compile_expr(value)?;
                let value_type = value.get_type();

                if let Some(fn_value) = &self.get_fn_value() {
                    let llvm_var = self.locals.load(fn_value, name);
                    let pointer = if let Some(llvm_var) = llvm_var {
                        llvm_var.pointer_value()
                    } else {
                        self.builder
                            .build_alloca(value_type.to_basic_type(self.context), name)
                    };
                    self.builder.build_store(pointer, value.to_basic_value());
                    self.locals.set(fn_value, name, (value_type, pointer));
                } else {
                    let global =
                        self.module
                            .add_global(value_type.to_basic_type(self.context), None, name);
                    global.set_unnamed_addr(true);
                    global.set_initializer(&value.to_basic_value());
                    let pointer = global.as_pointer_value();
                    self.globals.set(name, (value_type, pointer));
                }
                Ok(())
            }
            StatementType::Return { value } => {
                // Outside function
                if self.get_fn_value().is_none() {
                    return Err(LLVMCompileError::new(
                        self.get_source_location(),
                        LLVMCompileErrorType::SyntaxError("'return' outside function".to_string()),
                    ));
                }
                if let Some(value) = value {
                    let return_value = self.compile_expr(value)?;

                    if return_value.get_type() == ValueType::Void {
                        self.builder.build_return(None);
                    } else {
                        // Type check
                        let fn_type = self
                            .get_fn_value()
                            .unwrap()
                            .get_type()
                            .get_return_type()
                            .unwrap();
                        let value_type = return_value.to_basic_value().get_type();
                        if fn_type != value_type {
                            return Err(LLVMCompileError::new(
                                self.get_source_location(),
                                LLVMCompileErrorType::TypeError(
                                    format!("{:?}", fn_type),
                                    format!("{:?}", value_type),
                                ),
                            ));
                        }

                        return_value.invoke_handler(
                            ValueHandler::new()
                                .handle_int(&|_, value| self.builder.build_return(Some(&value)))
                                .handle_float(&|_, value| self.builder.build_return(Some(&value))),
                        );
                    }
                } else {
                    self.builder.build_return(None);
                }
                Ok(())
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
                    return Err(LLVMCompileError::new(
                        self.get_source_location(),
                        LLVMCompileErrorType::NotImplemented(Some(
                            "Import is not implemented".to_string(),
                        )),
                    ));
                }
                Ok(())
            }
            StatementType::If { test, body, orelse } => {
                let cond = self.compile_expr(test)?;

                match orelse {
                    None /*Only if:*/ => {
                        self.compile_stmt_conditional(cond, body, None)?;
                    }
                    Some(statements) => {
                        self.compile_stmt_conditional(cond, body, Some(statements))?;
                    }
                }
                Ok(())
            }
            StatementType::While { test, body, orelse } => {
                self.compile_stmt_while(test, body, orelse)?;
                Ok(())
            }
            _ => Err(LLVMCompileError::new(
                self.get_source_location(),
                LLVMCompileErrorType::NotImplemented(Some(format!("{:?}", stmt))),
            )),
        }
    }

    fn compile_stmt_function_def(
        &mut self,
        name: &String,
        args: &Box<ast::Parameters>,
        body: &ast::Suite,
        returns: &Option<ast::Expression>,
    ) -> Result<(), LLVMCompileError> {
        let mut args_vec: Vec<BasicTypeEnum> = vec![];
        let mut arg_names = vec![];

        for arg in args.args.iter() {
            arg_names.push(&arg.arg);
            if arg.annotation.is_none() {
                panic!("You must provide type hint for args");
            }
            let arg_type;
            match &arg.annotation.as_ref().unwrap().node {
                ast::ExpressionType::Identifier { name } => {
                    arg_type = name;
                }
                _ => {
                    panic!("Unknown return annotation node");
                }
            }
            match arg_type.as_str() {
                "int" => args_vec.push(self.context.i16_type().into()),
                "float" => args_vec.push(self.context.f32_type().into()),
                _ => panic!("Unknown argument type {}", arg_type),
            }
        }

        let mut return_type = &String::new();
        if let Some(annotation) = returns {
            match &annotation.node {
                ast::ExpressionType::Identifier { name } => {
                    return_type = name;
                }
                ast::ExpressionType::None => {}
                _ => {
                    panic!("Unknown return annotation node");
                }
            }
        }

        let f = match return_type.as_str() {
            "int8" => self.module.add_function(
                name,
                self.context.i8_type().fn_type(&args_vec, false),
                None,
            ),
            "int" => self.module.add_function(
                name,
                self.context.i16_type().fn_type(&args_vec, false),
                None,
            ),
            "float" => self.module.add_function(
                name,
                self.context.f32_type().fn_type(&args_vec, false),
                None,
            ),
            "" | "None" => self.module.add_function(
                name,
                self.context.void_type().fn_type(&args_vec, false),
                None,
            ),

            _ => panic!("Unknown return type {}", return_type),
        };
        let bb = self.context.append_basic_block(f, "");

        self.builder.position_at_end(bb);

        self.set_fn_value(Some(f));
        self.locals.create(self.get_fn_value().unwrap());

        for (i, bv) in f.get_param_iter().enumerate() {
            let v = if bv.is_int_value() {
                bv.into_int_value().set_name(arg_names[i]);
                Value::I16 {
                    value: bv.into_int_value(),
                }
            } else if bv.is_float_value() {
                bv.into_float_value().set_name(arg_names[i]);
                Value::F32 {
                    value: bv.into_float_value(),
                }
            } else {
                panic!("NotImplemented function argument type")
            };
            let pointer = self
                .builder
                .build_alloca(v.get_type().to_basic_type(self.context), arg_names[i]);
            self.builder.build_store(pointer, bv);
            self.locals.set(
                &self.get_fn_value().unwrap(),
                arg_names[i],
                (v.get_type(), pointer),
            );
        }

        for statement in body.iter() {
            self.compile_stmt(statement)?;
        }

        if f.get_type().get_return_type().is_none() && bb.get_terminator().is_none() {
            self.builder.build_return(None);
        }

        self.set_fn_value(None);
        Ok(())
    }

    fn compile_stmt_conditional(
        &mut self,
        cond: Value<'ctx>,
        body: &Vec<ast::Statement>,
        orelse: Option<&Vec<ast::Statement>>,
    ) -> Result<(), LLVMCompileError> {
        let parent = self.get_fn_value().unwrap();

        let cond_bb = self.context.append_basic_block(parent, "if.cond");
        let then_bb = self.context.append_basic_block(parent, "if.then");
        let else_bb = self.context.append_basic_block(parent, "if.else");

        let cond = cond.invoke_handler(
            ValueHandler::new()
                .handle_bool(&|_, value| value)
                // In Python, all integers except 0 are considered true.
                .handle_int(&|value, _| {
                    let a = value;
                    let b = Value::I16 {
                        value: self.context.i16_type().const_zero(),
                    };

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
                                b.invoke_handler(ValueHandler::new().handle_float(
                                    &|_, rhs_value| Value::Bool {
                                        value: self.builder.build_float_compare(
                                            FloatPredicate::ONE,
                                            lhs_value,
                                            rhs_value,
                                            "a",
                                        ),
                                    },
                                ))
                            }),
                    );

                    c.invoke_handler(ValueHandler::new().handle_bool(&|_, value| value))
                })
                // In Python, all float numbers except 0.0 are considered true.
                .handle_float(&|value, _| {
                    let a = value;
                    let b = Value::F32 {
                        value: self.context.f32_type().const_zero(),
                    };

                    // This LLVM expression is same as `lhs_value != 0.0`
                    // Therefore all float numbers except 0 are considered true.
                    let c = a.invoke_handler(ValueHandler::new().handle_float(&|_, lhs_value| {
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
                    }));

                    c.invoke_handler(ValueHandler::new().handle_bool(&|_, value| value))
                }),
        );

        self.builder
            .build_conditional_branch(cond, then_bb, else_bb);

        self.builder.position_at_end(then_bb);
        for statement in body.iter() {
            self.compile_stmt(statement)?;
        }
        self.builder.build_unconditional_branch(cond_bb);

        let _then_bb = self.builder.get_insert_block().unwrap();

        self.builder.position_at_end(else_bb);

        match orelse {
            Some(statements) => {
                for statement in statements.iter() {
                    self.compile_stmt(statement)?;
                }
            }
            None => {}
        }

        self.builder.build_unconditional_branch(cond_bb);

        let _else_bb = self.builder.get_insert_block().unwrap();

        self.builder.position_at_end(cond_bb);
        Ok(())
    }

    fn compile_stmt_while(
        &mut self,
        test: &ast::Expression,
        body: &ast::Suite,
        orelse: &Option<ast::Suite>,
    ) -> Result<(), LLVMCompileError> {
        let parent = self.get_fn_value().unwrap();

        let while_bb = self.context.append_basic_block(parent, "while.cond");
        let loop_bb = self.context.append_basic_block(parent, "while.body");
        let else_bb = self.context.append_basic_block(parent, "while.else");
        let after_bb = self.context.append_basic_block(parent, "while.after");

        self.builder.build_unconditional_branch(while_bb);
        self.builder.position_at_end(while_bb);

        let start = self.compile_expr(test)?;
        let cond = start.invoke_handler(
            ValueHandler::new()
                .handle_bool(&|_, value| value)
                // In Python, all integers except 0 are considered true.
                .handle_int(&|value, _| {
                    let a = value;
                    let b = Value::I16 {
                        value: self.context.i16_type().const_zero(),
                    };

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
                                b.invoke_handler(ValueHandler::new().handle_float(
                                    &|_, rhs_value| Value::Bool {
                                        value: self.builder.build_float_compare(
                                            FloatPredicate::ONE,
                                            lhs_value,
                                            rhs_value,
                                            "a",
                                        ),
                                    },
                                ))
                            }),
                    );

                    c.invoke_handler(ValueHandler::new().handle_bool(&|_, value| value))
                })
                // In Python, all float numbers except 0.0 are considered true.
                .handle_float(&|value, _| {
                    let a = value;
                    let b = Value::F32 {
                        value: self.context.f32_type().const_zero(),
                    };

                    // This LLVM expression is same as `lhs_value != 0.0`
                    // Therefore all float numbers except 0 are considered true.
                    let c = a.invoke_handler(ValueHandler::new().handle_float(&|_, lhs_value| {
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
                    }));

                    c.invoke_handler(ValueHandler::new().handle_bool(&|_, value| value))
                }),
        );

        self.builder
            .build_conditional_branch(cond, loop_bb, else_bb);
        self.builder.position_at_end(loop_bb);
        for statement in body.iter() {
            self.compile_stmt(statement)?;
        }
        self.builder
            .build_conditional_branch(cond, while_bb, else_bb);

        self.builder.position_at_end(else_bb);
        match orelse {
            Some(statements) => {
                for statement in statements.iter() {
                    self.compile_stmt(statement)?;
                }
            }
            None => {}
        }

        self.builder.build_unconditional_branch(after_bb);
        self.builder.position_at_end(after_bb);
        Ok(())
    }
}
