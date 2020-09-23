use either::Either;
use inkwell::values::{BasicValueEnum, InstructionOpcode};
use inkwell::{FloatPredicate, IntPredicate};

use dsp_compiler_error::{err, LLVMCompileError, LLVMCompileErrorType};
use dsp_compiler_mangler::mangling;
use dsp_compiler_value::convert::{truncate_bigint_to_u64, try_get_constant_string};
use dsp_compiler_value::value::{Value, ValueHandler, ValueType};
use dsp_python_parser::ast;

use crate::CodeGen;

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    pub fn compile_expr(
        &mut self,
        expr: &ast::Expression,
    ) -> Result<Value<'ctx>, LLVMCompileError> {
        self.set_loc(expr.location);

        use dsp_python_parser::ast::ExpressionType;
        match &expr.node {
            ExpressionType::Number { value } => match value {
                ast::Number::Integer { value } => {
                    let value = Value::I16 {
                        value: self
                            .context
                            .i16_type()
                            .const_int(truncate_bigint_to_u64(value), true),
                    };
                    Ok(value)
                }
                ast::Number::Float { value } => {
                    let value = Value::F32 {
                        value: self.context.f32_type().const_float(*value),
                    };
                    Ok(value)
                }
                ast::Number::Complex { real: _, imag: _ } => err!(
                    self,
                    LLVMCompileErrorType::NotImplemented,
                    "Imaginary numbers are not supported."
                ),
            },
            ExpressionType::String { value } => {
                let v = try_get_constant_string(value).unwrap();
                if self._fn_value.is_some() {
                    let value = Value::Str {
                        value: self
                            .builder
                            .build_global_string_ptr(&v, ".str")
                            .as_pointer_value(),
                    };
                    Ok(value)
                } else {
                    err!(
                        self,
                        LLVMCompileErrorType::NotImplemented,
                        "String expression in global scope is not implemented."
                    )
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
            ExpressionType::Identifier { name } => {
                let (value_type, pointer_value) = if let Some(fn_value) = self._fn_value {
                    let llvm_variable = self.locals.load(&fn_value, name);
                    if let Some(llvm_variable) = llvm_variable {
                        llvm_variable
                    } else {
                        let llvm_variable = self.globals.load(name);
                        if let Some(llvm_variable) = llvm_variable {
                            llvm_variable
                        } else {
                            return err!(self, LLVMCompileErrorType::NameError, name);
                        }
                    }
                } else {
                    let llvm_variable = self.globals.load(name);
                    if let Some(llvm_variable) = llvm_variable {
                        llvm_variable
                    } else {
                        return err!(self, LLVMCompileErrorType::NameError, name);
                    }
                };
                let value = Value::from_basic_value(
                    *value_type,
                    self.builder.build_load(*pointer_value, name),
                );
                Ok(value)
            }
            ExpressionType::Binop { a, op, b } => {
                let a = self.compile_expr(a)?;
                let b = self.compile_expr(b)?;
                self.compile_op(a, op, b)
            }
            ExpressionType::Compare { vals, ops } => self.compile_comparison(vals, ops),
            ExpressionType::Unop { op, a } => match &a.node {
                ExpressionType::Number { value } => match value {
                    ast::Number::Integer { value } => match op {
                        ast::UnaryOperator::Neg => {
                            let value = Value::I16 {
                                value: self
                                    .context
                                    .i16_type()
                                    .const_int(truncate_bigint_to_u64(&-value), true),
                            };
                            Ok(value)
                        }
                        _ => {
                            return err!(
                                self,
                                LLVMCompileErrorType::NotImplemented,
                                format!("Unimplemented unop {:?} for integer", op)
                            );
                        }
                    },
                    ast::Number::Float { value } => match op {
                        ast::UnaryOperator::Neg => {
                            let value = Value::F32 {
                                value: self.context.f32_type().const_float(-value.clone()),
                            };
                            Ok(value)
                        }
                        _ => {
                            return err!(
                                self,
                                LLVMCompileErrorType::NotImplemented,
                                format!("Unimplemented unop {:?} for floating number", op)
                            );
                        }
                    },
                    ast::Number::Complex { real: _, imag: _ } => {
                        return err!(
                            self,
                            LLVMCompileErrorType::NotImplemented,
                            format!("Complex numbers are not implemented.")
                        );
                    }
                },
                _ => {
                    return err!(
                        self,
                        LLVMCompileErrorType::NotImplemented,
                        format!("unary operator for {:?} is not implemented.", a.node)
                    );
                }
            },
            ExpressionType::True => Ok(Value::Bool {
                value: self.context.bool_type().const_int(1, false),
            }),
            ExpressionType::False => Ok(Value::Bool {
                value: self.context.bool_type().const_int(0, false),
            }),
            ExpressionType::None => Ok(Value::Void),
            _ => err!(
                self,
                LLVMCompileErrorType::NotImplemented,
                format!("{:?}", expr)
            ),
        }
    }

    fn compile_expr_call(
        &mut self,
        func: &Box<ast::Expression>,
        args: &Vec<ast::Expression>,
    ) -> Result<Value<'ctx>, LLVMCompileError> {
        let func_name = match &func.node {
            ast::ExpressionType::Identifier { name } => name.to_string(),
            _ => {
                return err!(
                    self,
                    LLVMCompileErrorType::NotImplemented,
                    "Calling method is not implemented."
                );
            }
        };

        // Compile the first argument to get type signature
        let first_arg = self.compile_expr(args.clone().first().unwrap())?;

        let func = match self.get_function(&func_name) {
            Some(f) => f,
            None => {
                // Simple mangling from the type of the first argument
                let func_name_mangled = mangling(&func_name, first_arg.get_type());
                if let Some(f) = self.get_function(&func_name_mangled) {
                    f
                } else {
                    return err!(self, LLVMCompileErrorType::NameError, &func_name);
                }
            }
        };

        let args_proto = func.get_params();

        let mut args_value: Vec<BasicValueEnum> = vec![];

        for (i, expr_proto) in args.iter().zip(args_proto.iter()).enumerate() {
            let expr = expr_proto.0;
            let proto = expr_proto.1;

            // Prevent compile the same argument twice
            let value = if i == 0 {
                first_arg
            } else {
                self.compile_expr(expr)?
            };

            // Convert the type of argument according to the signature
            match value {
                Value::I8 { value } => {
                    let cast = self.builder.build_int_cast(
                        value,
                        proto.get_type().into_int_type(),
                        "icast",
                    );
                    args_value.push(BasicValueEnum::IntValue(cast))
                }
                Value::I16 { value } => {
                    let cast = self.builder.build_int_truncate(
                        value,
                        proto.get_type().into_int_type(),
                        "itrunc",
                    );
                    args_value.push(BasicValueEnum::IntValue(cast))
                }
                Value::F32 { value } => args_value.push(BasicValueEnum::FloatValue(value)),
                Value::Str { value } => args_value.push(BasicValueEnum::PointerValue(value)),
                _ => {
                    return err!(
                        self,
                        LLVMCompileErrorType::NotImplemented,
                        format!("Unimplemented argument type '{:?}'", value.get_type())
                    );
                }
            }
        }

        let res = self.builder.build_call(func, args_value.as_slice(), "call");
        res.set_tail_call(true);

        // Returned value
        let value = match res.try_as_basic_value() {
            // Return type
            Either::Left(bv) => {
                let vt = if bv.is_int_value() {
                    let iv = bv.into_int_value();

                    match iv.get_type().get_bit_width() {
                        8 => ValueType::I8,
                        16 => ValueType::I16,
                        _ => unreachable!(),
                    }
                } else if bv.is_float_value() {
                    ValueType::F32
                } else {
                    unreachable!()
                };
                Value::from_basic_value(vt, bv)
            }
            Either::Right(_) => Value::Void,
        };
        Ok(value)
    }

    fn compile_op(
        &mut self,
        a: Value<'ctx>,
        op: &ast::Operator,
        b: Value<'ctx>,
    ) -> Result<Value<'ctx>, LLVMCompileError> {
        use dsp_python_parser::ast::Operator;
        Ok(a.invoke_handler(
            ValueHandler::new()
                .handle_int(&|_, lhs_value| {
                    b.invoke_handler(
                        ValueHandler::new()
                            // Between int and int
                            .handle_int(&|_, rhs_value| {
                                // Div operator to int returns a float.
                                if op == &Operator::Div {
                                    return Value::F32 {
                                        value: self.builder.build_float_div(
                                            lhs_value
                                                .const_signed_to_float(self.context.f32_type()),
                                            rhs_value
                                                .const_signed_to_float(self.context.f32_type()),
                                            "div",
                                        ),
                                    };
                                }
                                Value::I16 {
                                    value: match op {
                                        Operator::Add {} => {
                                            self.builder.build_int_add(lhs_value, rhs_value, "add")
                                        }
                                        Operator::Sub {} => {
                                            self.builder.build_int_sub(lhs_value, rhs_value, "sub")
                                        }
                                        Operator::Mult => {
                                            self.builder.build_int_mul(lhs_value, rhs_value, "mul")
                                        }
                                        Operator::Div => {
                                            // In Python, dividing int by int returns a float,
                                            // which is implemented above.
                                            unimplemented!()
                                        }
                                        Operator::FloorDiv => self
                                            .builder
                                            .build_int_signed_div(lhs_value, rhs_value, "fld"),
                                        Operator::Mod => self
                                            .builder
                                            .build_int_signed_rem(lhs_value, rhs_value, "mod"),
                                        _ => panic!("Unimplemented {:?} operator for i16", op),
                                    },
                                }
                            })
                            // Between int and float
                            .handle_float(&|_, rhs_value| Value::F32 {
                                value: match op {
                                    Operator::Mult => {
                                        let cast = self
                                            .builder
                                            .build_cast(
                                                InstructionOpcode::SIToFP,
                                                lhs_value,
                                                self.context.f32_type(),
                                                "sitofp",
                                            )
                                            .into_float_value();
                                        self.builder.build_float_mul(cast, rhs_value, "mul")
                                    }
                                    _ => panic!("Unimplemented {:?} operator for i16 and f32", op),
                                },
                            }),
                    )
                })
                .handle_float(&|_, lhs_value| {
                    b.invoke_handler(
                        ValueHandler::new()
                            // Between float and float
                            .handle_float(&|_, rhs_value| Value::F32 {
                                value: match op {
                                    Operator::Add {} => {
                                        self.builder.build_float_add(lhs_value, rhs_value, "add")
                                    }
                                    Operator::Sub {} => {
                                        self.builder.build_float_sub(lhs_value, rhs_value, "sub")
                                    }
                                    Operator::Mult => {
                                        self.builder.build_float_mul(lhs_value, rhs_value, "mul")
                                    }
                                    Operator::Div => {
                                        self.builder.build_float_div(lhs_value, rhs_value, "div")
                                    }
                                    Operator::FloorDiv => unimplemented!(),
                                    Operator::Mod => {
                                        self.builder.build_float_rem(lhs_value, rhs_value, "mod")
                                    }
                                    _ => panic!("Unimplemented {:?} operator for f32", op),
                                },
                            })
                            // Between float and int
                            .handle_int(&|_, rhs_value| Value::F32 {
                                value: match op {
                                    Operator::Mult => {
                                        let cast = self
                                            .builder
                                            .build_cast(
                                                InstructionOpcode::SIToFP,
                                                rhs_value,
                                                self.context.f32_type(),
                                                "sitofp",
                                            )
                                            .into_float_value();
                                        self.builder.build_float_mul(lhs_value, cast, "mul")
                                    }
                                    _ => panic!("Unimplemented {:?} operator for f32 and i16", op),
                                },
                            }),
                    )
                }),
        ))
    }

    fn compile_comparison(
        &mut self,
        vals: &[ast::Expression],
        ops: &[ast::Comparison],
    ) -> Result<Value<'ctx>, LLVMCompileError> {
        if ops.len() > 1 || vals.len() > 2 {
            return err!(
                self,
                LLVMCompileErrorType::NotImplemented,
                "Chained comparison is not implemented."
            );
        }

        let a = self.compile_expr(vals.first().unwrap())?;
        let b = self.compile_expr(vals.last().unwrap())?;

        Ok(a.invoke_handler(
            ValueHandler::new()
                .handle_int(&|_, lhs_value| {
                    b.invoke_handler(ValueHandler::new().handle_int(&|_, rhs_value| {
                        let int_predicate = match ops.first().unwrap() {
                            ast::Comparison::Equal => IntPredicate::EQ,
                            ast::Comparison::NotEqual => IntPredicate::NE,
                            ast::Comparison::Greater => IntPredicate::SGT,
                            ast::Comparison::Less => IntPredicate::SLT,
                            ast::Comparison::GreaterOrEqual => IntPredicate::SGE,
                            ast::Comparison::LessOrEqual => IntPredicate::SLE,
                            _ => panic!(
                                "Unsupported {:?} comparison operator for integer",
                                ops.first().unwrap()
                            ),
                        };
                        Value::Bool {
                            value: self.builder.build_int_compare(
                                int_predicate,
                                lhs_value,
                                rhs_value,
                                "a",
                            ),
                        }
                    }))
                })
                .handle_float(&|_, lhs_value| {
                    b.invoke_handler(ValueHandler::new().handle_float(&|_, rhs_value| {
                        let float_predicate = match ops.first().unwrap() {
                            ast::Comparison::Equal => FloatPredicate::OEQ,
                            ast::Comparison::NotEqual => FloatPredicate::ONE,
                            ast::Comparison::Greater => FloatPredicate::OGT,
                            ast::Comparison::Less => FloatPredicate::OLT,
                            ast::Comparison::GreaterOrEqual => FloatPredicate::OGE,
                            ast::Comparison::LessOrEqual => FloatPredicate::OLE,
                            _ => panic!(
                                "Unsupported {:?} comparison operator for floating number",
                                ops.first().unwrap()
                            ),
                        };
                        Value::Bool {
                            value: self.builder.build_float_compare(
                                float_predicate,
                                lhs_value,
                                rhs_value,
                                "a",
                            ),
                        }
                    }))
                }),
        ))
    }
}
