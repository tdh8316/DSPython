use either::Either;
use inkwell::values::BasicValueEnum;
use rustpython_parser::ast;

use crate::compiler::Compiler;
use crate::compiler::mangle::mangling;
use crate::value::{Value, ValueType};
use crate::value::convert::{truncate_bigint_to_u64, try_get_constant_string};

pub trait CGExpr<'a, 'ctx> {
    fn compile_expr(&mut self, expr: &ast::Expression) -> Value<'ctx>;
    fn compile_expr_call(
        &mut self,
        func: &Box<ast::Expression>,
        args: &Vec<ast::Expression>,
    ) -> Value<'ctx>;
}

impl<'a, 'ctx> CGExpr<'a, 'ctx> for Compiler<'a, 'ctx> {
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
                        "{:?}\nNotImplemented builder for imaginary number",
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
                    // TODO: Global string builder
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
            ExpressionType::Compare { vals, ops } => self.compile_comparison(vals, ops),
            ExpressionType::None => Value::Void,
            ExpressionType::True => Value::Bool {
                value: self.context.bool_type().const_int(1, false),
            },
            ExpressionType::False => Value::Bool {
                value: self.context.bool_type().const_int(0, false),
            },
            ExpressionType::Unop { op, a } => {
                match &a.node {
                    ExpressionType::Number { value } => match value {
                        ast::Number::Integer { value } => {
                            match op {
                                ast::UnaryOperator::Neg => Value::I16 {
                                    value: self
                                        .context
                                        .i16_type()
                                        .const_int(truncate_bigint_to_u64(&-value), true),
                                },
                                _ => {
                                    panic!("NotImplemented unop for i16")
                                }
                            }
                        }
                        ast::Number::Float { value } => {
                            match op {
                                ast::UnaryOperator::Neg => Value::F32 {
                                    value: self.context.f32_type().const_float(-value.clone()),
                                },
                                _ => {
                                    panic!("NotImplemented unop for f32")
                                }
                            }
                        }
                        ast::Number::Complex { real: _, imag: _ } => {
                            panic!(
                                "{:?}\nNotImplemented builder for imaginary number",
                                self.current_source_location
                            );
                        }
                    },
                    _ => {
                        panic!("NotImplemented type for unop")
                    }
                }
            }
            _ => {
                panic!(
                    "{:?}\nNotImplemented expression {:?}",
                    self.current_source_location, expr.node,
                );
            }
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
}
