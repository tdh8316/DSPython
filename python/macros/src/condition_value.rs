/// Condition value handler
#[macro_export]
macro_rules! cvhandler {
    ($self:ident) => {{
        ValueHandler::new()
            .handle_bool(&|_, value| value)
            // In Python, all integers except 0 are considered true.
            .handle_int(&|value, _| {
                let a = value;
                let b = Value::I16 {
                    value: $self.context.i16_type().const_zero(),
                };

                // This LLVM expression is same as `lhs_value != 0`
                // Therefore all integers except 0 are considered true.
                let c = a.invoke_handler(
                    ValueHandler::new()
                        .handle_int(&|_, lhs_value| {
                            b.invoke_handler(ValueHandler::new().handle_int(&|_, rhs_value| {
                                Value::Bool {
                                    value: $self.builder.build_int_compare(
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
                                    value: $self.builder.build_float_compare(
                                        FloatPredicate::ONE,
                                        lhs_value,
                                        rhs_value,
                                        "a",
                                    ),
                                }
                            }))
                        }),
                );

                c.invoke_handler(ValueHandler::new().handle_bool(&|_, value| value))
            })
            // In Python, all float numbers except 0.0 are considered true.
            .handle_float(&|value, _| {
                let a = value;
                let b = Value::F32 {
                    value: $self.context.f32_type().const_zero(),
                };

                // This LLVM expression is same as `lhs_value != 0.0`
                // Therefore all float numbers except 0 are considered true.
                let c = a.invoke_handler(ValueHandler::new().handle_float(&|_, lhs_value| {
                    b.invoke_handler(ValueHandler::new().handle_float(&|_, rhs_value| {
                        Value::Bool {
                            value: $self.builder.build_float_compare(
                                FloatPredicate::ONE,
                                lhs_value,
                                rhs_value,
                                "a",
                            ),
                        }
                    }))
                }));

                c.invoke_handler(ValueHandler::new().handle_bool(&|_, value| value))
            })
    }};
}
