use inkwell::values::BasicMetadataValueEnum;
use inkwell::IntPredicate;
use rustpython_parser::ast;

use crate::codegen::errors::CodeGenError;
use crate::codegen::symbol_table::SymbolValueTrait;
use crate::codegen::value::{truncate_bigint_to_u64, Value, ValueType};
use crate::codegen::vargs::get_vargs_types;
use crate::codegen::CodeGen;
use crate::compiler::mangler::get_mangled_func_name;

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    pub fn emit_expr(&mut self, expr: &ast::Expr) -> Result<Value<'ctx>, CodeGenError> {
        self.set_source_location(expr.location);
        use ast::ExprKind::*;
        match &expr.node {
            Constant { value, kind } => self.emit_constant(value, kind),
            Name { id, .. } => self.emit_name(id),
            BinOp { left, op, right } => self.emit_bin_op(left, op, right),
            Call { func, args, .. } => self.emit_call(func, args),
            Compare {
                left,
                ops,
                comparators,
            } => self.emit_compare(left, ops, comparators),
            UnaryOp { op, operand } => self.emit_unary_op(op, operand),

            _ => Err(CodeGenError::Unimplemented(format!("expr: {:#?}", expr))),
        }
    }

    fn emit_unary_op(
        &mut self,
        op: &ast::Unaryop,
        operand: &ast::Expr,
    ) -> Result<Value<'ctx>, CodeGenError> {
        let operand = self.emit_expr(operand)?;

        return match op {
            ast::Unaryop::USub => {
                // Ok(Value::I32 {
                //     value: self.builder.build_int_sub(self.context.i32_type().const_int(0, true), operand.to_basic_value().into_int_value(), "neg"),
                // })
                Ok(Value::I32 {
                    value: self.builder.build_int_sub(
                        self.context.i32_type().const_int(0, true),
                        operand.to_basic_value().into_int_value(),
                        "neg",
                    ),
                })
            }
            _ => Err(CodeGenError::Unimplemented(format!("unary op: {:#?}", op))),
        };
    }

    fn emit_compare(
        &mut self,
        left: &ast::Expr,
        ops: &[ast::Cmpop],
        comparators: &Vec<ast::Expr>,
    ) -> Result<Value<'ctx>, CodeGenError> {
        if ops.len() > 1 || comparators.len() > 2 {
            return Err(CodeGenError::Unimplemented(
                "Chained comparisons are not supported".to_string(),
            ));
        }

        let left = self.emit_expr(left)?;
        let right = self.emit_expr(&comparators[0])?;

        let value = match (left.get_type(), right.get_type()) {
            (ValueType::I32, ValueType::I32) => {
                let operator = match &ops[0] {
                    ast::Cmpop::Eq => IntPredicate::EQ,
                    ast::Cmpop::NotEq => IntPredicate::NE,
                    ast::Cmpop::Lt => IntPredicate::SLT,
                    ast::Cmpop::LtE => IntPredicate::SLE,
                    ast::Cmpop::Gt => IntPredicate::SGT,
                    ast::Cmpop::GtE => IntPredicate::SGE,
                    _ => {
                        return Err(CodeGenError::Unimplemented(format!(
                            "{:?} for i32 and i32 is not implemented",
                            ops[0]
                        )));
                    }
                };
                self.builder.build_int_compare(
                    operator,
                    left.to_basic_value().into_int_value(),
                    right.to_basic_value().into_int_value(),
                    "eq",
                )
            }
            _ => {
                return Err(CodeGenError::CompileError(format!(
                    "Cannot compare {:?} and {:?}",
                    left.get_type(),
                    right.get_type()
                )));
            }
        };

        Ok(Value::Bool { value })
    }

    fn emit_call(
        &mut self,
        func: &ast::Expr,
        args: &Vec<ast::Expr>,
    ) -> Result<Value<'ctx>, CodeGenError> {
        let mut args_values: Vec<Value<'ctx>> = Vec::new();

        // Evaluate arguments.
        for arg_expr in args {
            args_values.push(self.emit_expr(arg_expr)?);
        }

        let func_name = get_symbol_str_from_expr(func)?;
        let func = if let Some(func) = self.module.get_function(func_name.as_str()) {
            func
        } else {
            // If the function is not found, mangle the name and try again.
            if let Some(func) = self
                .module
                .get_function(get_mangled_func_name(func_name.as_str(), &args_values).as_str())
            {
                func
            } else {
                return Err(CodeGenError::NameError(format!(
                    "name '{}' is not defined",
                    func_name
                )));
            }
        };

        let mut args_basic_values: Vec<BasicMetadataValueEnum> = args_values
            .clone()
            .into_iter()
            .map(|arg| BasicMetadataValueEnum::from(arg.to_basic_value()))
            .collect::<Vec<BasicMetadataValueEnum>>();

        // If the function has variadic arguments
        if func.get_type().is_var_arg() {
            // Add the type of the variadic arguments to the argument list.
            args_basic_values.insert(
                0,
                BasicMetadataValueEnum::from(
                    self.builder
                        .build_global_string_ptr(get_vargs_types(args_values).as_str(), "t_vargs")
                        .as_pointer_value(),
                ),
            );
        }

        let call = self
            .builder
            .build_call(func, args_basic_values.as_slice(), func_name.as_str());
        call.set_tail_call(true);

        // Evaluate the return value of the function.
        let return_value = match call.try_as_basic_value().left() {
            Some(bv) => Value::from_basic_value(ValueType::from_basic_type(bv.get_type()), bv),
            None => {
                // The function does not return a value.
                // Return None.
                Value::None
            }
        };

        Ok(return_value)
    }

    fn emit_bin_op(
        &mut self,
        left: &ast::Expr,
        op: &ast::Operator,
        right: &ast::Expr,
    ) -> Result<Value<'ctx>, CodeGenError> {
        let result_value = match op {
            ast::Operator::Add => self.emit_add(left, right)?,
            ast::Operator::Sub => self.emit_sub(left, right)?,
            ast::Operator::Mult => self.emit_mult(left, right)?,
            ast::Operator::Div => self.emit_div(left, right)?,
            ast::Operator::Mod => self.emit_mod(left, right)?,
            _ => return Err(CodeGenError::Unimplemented(format!("operator: {:#?}", op))),
        };
        Ok(result_value)
    }

    fn emit_mod(
        &mut self,
        left: &ast::Expr,
        right: &ast::Expr,
    ) -> Result<Value<'ctx>, CodeGenError> {
        let left_value = self.emit_expr(left)?;
        let right_value = self.emit_expr(right)?;

        let result_value = match (left_value.get_type(), right_value.get_type()) {
            (ValueType::I32, ValueType::I32) => {
                let value = self.builder.build_int_signed_rem(
                    left_value.to_basic_value().into_int_value(),
                    right_value.to_basic_value().into_int_value(),
                    "mod",
                );
                Value::I32 { value }
            }
            (ValueType::F32, ValueType::F32) => {
                let value = self.builder.build_float_rem(
                    left_value.to_basic_value().into_float_value(),
                    right_value.to_basic_value().into_float_value(),
                    "mod",
                );
                Value::F32 { value }
            }
            (ValueType::I32, ValueType::F32) => {
                let left_value = self.builder.build_signed_int_to_float(
                    left_value.to_basic_value().into_int_value(),
                    self.context.f32_type(),
                    "i32_to_f32",
                );
                let value = self.builder.build_float_rem(
                    left_value,
                    right_value.to_basic_value().into_float_value(),
                    "mod",
                );
                Value::F32 { value }
            }
            (ValueType::F32, ValueType::I32) => {
                let right_value = self.builder.build_signed_int_to_float(
                    left_value.to_basic_value().into_int_value(),
                    self.context.f32_type(),
                    "i32_to_f32",
                );
                let value = self.builder.build_float_rem(
                    left_value.to_basic_value().into_float_value(),
                    right_value,
                    "mod",
                );
                Value::F32 { value }
            }
            _ => {
                return Err(CodeGenError::CompileError(format!(
                    "Unsupported operand type(s) for -: '{:?}' and '{:?}'",
                    left_value.get_type(),
                    right_value.get_type()
                )));
            }
        };

        Ok(result_value)
    }

    fn emit_div(
        &mut self,
        left: &ast::Expr,
        right: &ast::Expr,
    ) -> Result<Value<'ctx>, CodeGenError> {
        let left_value = self.emit_expr(left)?;
        let right_value = self.emit_expr(right)?;

        let result_value = match (left_value.get_type(), right_value.get_type()) {
            // In Python, / operator always returns a float.
            (ValueType::I32, ValueType::I32) => {
                let left_value = self.builder.build_signed_int_to_float(
                    left_value.to_basic_value().into_int_value(),
                    self.context.f32_type(),
                    "i32_to_f32",
                );
                let right_value = self.builder.build_signed_int_to_float(
                    right_value.to_basic_value().into_int_value(),
                    self.context.f32_type(),
                    "i32_to_f32",
                );
                let value = self.builder.build_float_div(left_value, right_value, "div");
                Value::F32 { value }
            }
            (ValueType::F32, ValueType::F32) => {
                let value = self.builder.build_float_div(
                    left_value.to_basic_value().into_float_value(),
                    right_value.to_basic_value().into_float_value(),
                    "div",
                );
                Value::F32 { value }
            }
            (ValueType::I32, ValueType::F32) => {
                let left_value = self.builder.build_signed_int_to_float(
                    left_value.to_basic_value().into_int_value(),
                    self.context.f32_type(),
                    "i32_to_f32",
                );
                let value = self.builder.build_float_div(
                    left_value,
                    right_value.to_basic_value().into_float_value(),
                    "div",
                );
                Value::F32 { value }
            }
            (ValueType::F32, ValueType::I32) => {
                let right_value = self.builder.build_signed_int_to_float(
                    left_value.to_basic_value().into_int_value(),
                    self.context.f32_type(),
                    "i32_to_f32",
                );
                let value = self.builder.build_float_div(
                    left_value.to_basic_value().into_float_value(),
                    right_value,
                    "div",
                );
                Value::F32 { value }
            }
            _ => {
                return Err(CodeGenError::CompileError(format!(
                    "Unsupported operand type(s) for /: '{:?}' and '{:?}'",
                    left_value.get_type(),
                    right_value.get_type()
                )));
            }
        };

        Ok(result_value)
    }

    fn emit_mult(
        &mut self,
        left: &ast::Expr,
        right: &ast::Expr,
    ) -> Result<Value<'ctx>, CodeGenError> {
        let left_value = self.emit_expr(left)?;
        let right_value = self.emit_expr(right)?;

        let result_value = match (left_value.get_type(), right_value.get_type()) {
            (ValueType::I32, ValueType::I32) => {
                let value = self.builder.build_int_mul(
                    left_value.to_basic_value().into_int_value(),
                    right_value.to_basic_value().into_int_value(),
                    "mul",
                );
                Value::I32 { value }
            }
            (ValueType::F32, ValueType::F32) => {
                let value = self.builder.build_float_mul(
                    left_value.to_basic_value().into_float_value(),
                    right_value.to_basic_value().into_float_value(),
                    "mul",
                );
                Value::F32 { value }
            }
            (ValueType::I32, ValueType::F32) => {
                let left_value = self.builder.build_signed_int_to_float(
                    left_value.to_basic_value().into_int_value(),
                    self.context.f32_type(),
                    "i32_to_f32",
                );
                let value = self.builder.build_float_mul(
                    left_value,
                    right_value.to_basic_value().into_float_value(),
                    "mul",
                );
                Value::F32 { value }
            }
            (ValueType::F32, ValueType::I32) => {
                let right_value = self.builder.build_signed_int_to_float(
                    left_value.to_basic_value().into_int_value(),
                    self.context.f32_type(),
                    "i32_to_f32",
                );
                let value = self.builder.build_float_mul(
                    left_value.to_basic_value().into_float_value(),
                    right_value,
                    "mul",
                );
                Value::F32 { value }
            }
            _ => {
                return Err(CodeGenError::CompileError(format!(
                    "Unsupported operand type(s) for *: '{:?}' and '{:?}'",
                    left_value.get_type(),
                    right_value.get_type()
                )));
            }
        };

        Ok(result_value)
    }

    fn emit_sub(
        &mut self,
        left: &ast::Expr,
        right: &ast::Expr,
    ) -> Result<Value<'ctx>, CodeGenError> {
        let left_value = self.emit_expr(left)?;
        let right_value = self.emit_expr(right)?;

        let result_value = match (left_value.get_type(), right_value.get_type()) {
            (ValueType::I32, ValueType::I32) => {
                let value = self.builder.build_int_sub(
                    left_value.to_basic_value().into_int_value(),
                    right_value.to_basic_value().into_int_value(),
                    "sub",
                );
                Value::I32 { value }
            }
            (ValueType::F32, ValueType::F32) => {
                let value = self.builder.build_float_sub(
                    left_value.to_basic_value().into_float_value(),
                    right_value.to_basic_value().into_float_value(),
                    "sub",
                );
                Value::F32 { value }
            }
            (ValueType::I32, ValueType::F32) => {
                let left_value = self.builder.build_signed_int_to_float(
                    left_value.to_basic_value().into_int_value(),
                    self.context.f32_type(),
                    "i32_to_f32",
                );
                let value = self.builder.build_float_sub(
                    left_value,
                    right_value.to_basic_value().into_float_value(),
                    "sub",
                );
                Value::F32 { value }
            }
            (ValueType::F32, ValueType::I32) => {
                let right_value = self.builder.build_signed_int_to_float(
                    left_value.to_basic_value().into_int_value(),
                    self.context.f32_type(),
                    "i32_to_f32",
                );
                let value = self.builder.build_float_sub(
                    left_value.to_basic_value().into_float_value(),
                    right_value,
                    "sub",
                );
                Value::F32 { value }
            }
            _ => {
                return Err(CodeGenError::CompileError(format!(
                    "Unsupported operand type(s) for -: '{:?}' and '{:?}'",
                    left_value.get_type(),
                    right_value.get_type()
                )));
            }
        };

        Ok(result_value)
    }

    fn emit_add(
        &mut self,
        left: &ast::Expr,
        right: &ast::Expr,
    ) -> Result<Value<'ctx>, CodeGenError> {
        let left_value = self.emit_expr(left)?;
        let right_value = self.emit_expr(right)?;

        let result_value = match (left_value.get_type(), right_value.get_type()) {
            (ValueType::I32, ValueType::I32) => {
                let value = self.builder.build_int_add(
                    left_value.to_basic_value().into_int_value(),
                    right_value.to_basic_value().into_int_value(),
                    "add",
                );
                Value::I32 { value }
            }
            (ValueType::F32, ValueType::F32) => {
                let value = self.builder.build_float_add(
                    left_value.to_basic_value().into_float_value(),
                    right_value.to_basic_value().into_float_value(),
                    "add",
                );
                Value::F32 { value }
            }
            (ValueType::I32, ValueType::F32) => {
                let left_value = self.builder.build_signed_int_to_float(
                    left_value.to_basic_value().into_int_value(),
                    self.context.f32_type(),
                    "i32_to_f32",
                );
                let value = self.builder.build_float_add(
                    left_value,
                    right_value.to_basic_value().into_float_value(),
                    "add",
                );
                Value::F32 { value }
            }
            (ValueType::F32, ValueType::I32) => {
                let right_value = self.builder.build_signed_int_to_float(
                    left_value.to_basic_value().into_int_value(),
                    self.context.f32_type(),
                    "i32_to_f32",
                );
                let value = self.builder.build_float_add(
                    left_value.to_basic_value().into_float_value(),
                    right_value,
                    "add",
                );
                Value::F32 { value }
            }
            _ => {
                return Err(CodeGenError::CompileError(format!(
                    "Unsupported operand type(s) for +: '{:?}' and '{:?}'",
                    left_value.get_type(),
                    right_value.get_type()
                )));
            }
        };

        Ok(result_value)
    }

    fn emit_name(&mut self, id: &String) -> Result<Value<'ctx>, CodeGenError> {
        if let Some(symbol) = self.symbol_tables.context().get_symbol(id) {
            Ok(Value::from_basic_value(
                symbol.value.get_type(),
                self.builder.build_load(symbol.value.get_pointer(), id),
            ))
        } else {
            Err(CodeGenError::NameError(format!(
                "name '{}' is not defined",
                id
            )))
        }
    }

    fn emit_constant(
        &self,
        value: &ast::Constant,
        _kind: &Option<String>,
    ) -> Result<Value<'ctx>, CodeGenError> {
        return match value {
            ast::Constant::None => Ok(Value::None),
            ast::Constant::Bool(bool) => Ok(Value::Bool {
                value: self.context.bool_type().const_int(*bool as u64, false),
            }),
            ast::Constant::Str(value) => Ok(Value::Str {
                value: self
                    .builder
                    .build_global_string_ptr(value, "str")
                    .as_pointer_value(),
            }),
            // ast::Constant::Bytes(_) => {}
            ast::Constant::Int(value) => Ok(Value::I32 {
                value: self
                    .context
                    .i32_type()
                    .const_int(truncate_bigint_to_u64(value), true),
            }),
            // ast::Constant::Tuple(_) => {}
            ast::Constant::Float(value) => Ok(Value::F32 {
                value: self.context.f32_type().const_float(*value),
            }),
            // ast::Constant::Complex { .. } => {}
            // ast::Constant::Ellipsis => {}
            _ => Err(CodeGenError::Unimplemented(format!("{:?}", value))),
        };
    }
}

pub fn get_symbol_str_from_expr(expr: &ast::Expr) -> Result<String, CodeGenError> {
    use ast::ExprKind::*;
    match &expr.node {
        Name { id, .. } => Ok(id.to_string()),
        _ => Err(CodeGenError::CompileError(format!(
            "Cannot get symbol name from {:?}",
            expr
        ))),
    }
}

pub fn get_value_type_from_annotation(annotation: &ast::Expr) -> Result<ValueType, CodeGenError> {
    match &annotation.node {
        ast::ExprKind::Name { id, .. } => match id.as_str() {
            "int" => Ok(ValueType::I32),
            "float" => Ok(ValueType::F32),
            "str" => Ok(ValueType::Str),
            "bool" => Ok(ValueType::Bool),
            _ => Err(CodeGenError::CompileError(format!(
                "Unsupported type {}",
                id
            ))),
        },
        _ => Err(CodeGenError::CompileError(format!(
            "Cannot determine type from {:?}",
            annotation
        ))),
    }
}
