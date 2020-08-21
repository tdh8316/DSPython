use either::Either;
use inkwell::values::BasicValueEnum;

use dsp_compiler_error::*;
use dsp_compiler_value::convert::{truncate_bigint_to_u64, try_get_constant_string};
use dsp_compiler_value::value::{Value, ValueType};
use dsp_python_parser::ast;

use crate::CodeGen;
use dsp_compiler_mangler::mangling;

pub trait CGExpr<'a, 'ctx> {
    fn compile_expr(&mut self, expr: &ast::Expression) -> Result<Value<'ctx>, LLVMCompileError>;
    fn compile_expr_call(
        &mut self,
        func: &Box<ast::Expression>,
        args: &Vec<ast::Expression>,
    ) -> Result<Value<'ctx>, LLVMCompileError>;
}

impl<'a, 'ctx> CGExpr<'a, 'ctx> for CodeGen<'a, 'ctx> {
    fn compile_expr(&mut self, expr: &ast::Expression) -> Result<Value<'ctx>, LLVMCompileError> {
        self.set_source_location(expr.location);

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
                ast::Number::Complex { real: _, imag: _ } => Err(LLVMCompileError::new(
                    self.get_source_location(),
                    LLVMCompileErrorType::NotImplemented(Some(
                        "Imaginary numbers are not supported.".parse().unwrap(),
                    )),
                )),
            },
            ExpressionType::String { value } => {
                let v = try_get_constant_string(value).unwrap();
                if self.get_fn_value().is_some() {
                    let value = Value::Str {
                        value: self
                            .builder
                            .build_global_string_ptr(v.as_str(), ".str")
                            .as_pointer_value(),
                    };
                    Ok(value)
                } else {
                    Err(LLVMCompileError::new(
                        self.get_source_location(),
                        LLVMCompileErrorType::NotImplemented(Some(
                            "String expression in global scope is not implemented."
                                .parse()
                                .unwrap(),
                        )),
                    ))
                }
            }
            ExpressionType::Call {
                function,
                args,
                keywords,
            } => {
                let _keywords = keywords;
                Ok(self.compile_expr_call(function, args)?)
            }
            ExpressionType::Binop { a, op, b } => {
                let a = self.compile_expr(a)?;
                let b = self.compile_expr(b)?;
                // self.compile_op(a, op, b)
                unimplemented!()
            }
            ExpressionType::Identifier { name } => {
                let (value_type, pointer_value) = if let Some(fn_value) = self.get_fn_value() {
                    let llvm_variable = self.locals.load(&fn_value, name);
                    if let Some(llvm_variable) = llvm_variable {
                        llvm_variable
                    } else {
                        let llvm_variable = self.globals.load(name);
                        if let Some(llvm_variable) = llvm_variable {
                            llvm_variable
                        } else {
                            return Err(LLVMCompileError::new(
                                self.get_source_location(),
                                LLVMCompileErrorType::NameError(name.to_string()),
                            ));
                        }
                    }
                } else {
                    let llvm_variable = self.globals.load(name);
                    if let Some(llvm_variable) = llvm_variable {
                        llvm_variable
                    } else {
                        return Err(LLVMCompileError::new(
                            self.get_source_location(),
                            LLVMCompileErrorType::NameError(String::from(name)),
                        ));
                    }
                };
                let value = Value::from_basic_value(
                    *value_type,
                    self.builder.build_load(*pointer_value, name),
                );
                Ok(value)
            }
            ExpressionType::None => Ok(Value::Void),
            ExpressionType::True => Ok(Value::Bool {
                value: self.context.bool_type().const_int(1, false),
            }),
            ExpressionType::False => Ok(Value::Bool {
                value: self.context.bool_type().const_int(0, false),
            }),
            _ => Err(LLVMCompileError::new(
                self.get_source_location(),
                LLVMCompileErrorType::NotImplemented(Some(format!("{:?}", expr))),
            )),
        }
    }

    fn compile_expr_call(
        &mut self,
        func: &Box<ast::Expression>,
        args: &Vec<ast::Expression>,
    ) -> Result<Value<'ctx>, LLVMCompileError> {
        let func_name = match &func.node {
            ast::ExpressionType::Identifier { name } => name,
            _ => {
                return Err(LLVMCompileError::new(
                    self.get_source_location(),
                    LLVMCompileErrorType::NotImplemented(Some(format!(
                        "Calling method is not implemented."
                    ))),
                ));
            }
        }
        .to_string();

        let first_arg = self.compile_expr(args.clone().first().unwrap())?;

        let func = match self.get_function(func_name.as_ref()) {
            Some(f) => f,
            None => {
                let func_name_mangled = mangling(&func_name, first_arg.get_type());
                self.get_function(func_name_mangled.as_ref()).expect(
                    format!(
                        "{:?}\nFunction '{}' is not defined",
                        self.get_source_location(),
                        func_name
                    )
                    .as_str(),
                )
            }
        };

        let args_proto = func.get_params();

        let mut args_value: Vec<BasicValueEnum> = vec![];

        for (i, expr_proto) in args.iter().zip(args_proto.iter()).enumerate() {
            let expr = expr_proto.0;
            let proto = expr_proto.1;
            let value = if i == 0 {
                first_arg
            } else {
                self.compile_expr(expr)?
            };
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
                _ => panic!(
                    "{:?}\nNotImplemented argument type",
                    self.get_source_location()
                ),
            }
        }

        let res = self.builder.build_call(func, args_value.as_slice(), "call");
        res.set_tail_call(true);

        let value = match res.try_as_basic_value() {
            // Return type
            Either::Left(bv) => Value::from_basic_value(
                if bv.is_int_value() {
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
                },
                bv,
            ),
            Either::Right(_) => Value::Void,
        };
        Ok(value)
    }
}
