use rustpython_parser::ast;

use crate::codegen::errors::CodeGenError;
use crate::codegen::symbol_table::SymbolValueTrait;
use crate::codegen::value::{truncate_bigint_to_u64, Value, ValueType};
use crate::codegen::CodeGen;

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    pub fn emit_expr(&mut self, expr: &ast::Expr) -> Result<Value<'ctx>, CodeGenError> {
        self.set_source_location(expr.location);
        use ast::ExprKind::*;
        match &expr.node {
            Constant { value, kind } => self.emit_constant(value, kind),
            Name { id, .. } => self.emit_name(id),
            BinOp { left, op, right } => self.emit_bin_op(left, op, right),

            _ => Err(CodeGenError::Unimplemented(format!("expr: {:#?}", expr))),
        }
    }

    fn emit_bin_op(&mut self, left: &ast::Expr, op: &ast::Operator, right: &ast::Expr) -> Result<Value<'ctx>, CodeGenError> {
        let result_value = match op {
            ast::Operator::Add => self.emit_add(left, right)?,
            _ => return Err(CodeGenError::Unimplemented(format!("operator: {:#?}", op))),
        };
        Ok(result_value)
    }

    fn emit_add(&mut self, left: &ast::Expr, right: &ast::Expr) -> Result<Value<'ctx>, CodeGenError> {
        let left_value = self.emit_expr(left)?;
        let right_value = self.emit_expr(right)?;

        let result_value = match (left_value.get_type(), right_value.get_type()) {
            (ValueType::I32, ValueType::I32) => {
                let value = self.builder.build_int_add(left_value.to_basic_value().into_int_value(), right_value.to_basic_value().into_int_value(), "add");
                Value::I32 { value }
            }
            (ValueType::F32, ValueType::F32) => {
                let value = self.builder.build_float_add(left_value.to_basic_value().into_float_value(), right_value.to_basic_value().into_float_value(), "add");
                Value::F32 { value }
            }
            (ValueType::I32, ValueType::F32) => {
                let left_value = self.builder.build_signed_int_to_float(left_value.to_basic_value().into_int_value(), self.context.f32_type(), "i32_to_f32");
                let value = self.builder.build_float_add(left_value, right_value.to_basic_value().into_float_value(), "add");
                Value::F32 { value }
            }
            (ValueType::F32, ValueType::I32) => {
                let right_value = self.builder.build_signed_int_to_float(left_value.to_basic_value().into_int_value(), self.context.f32_type(), "i32_to_f32");
                let value = self.builder.build_float_add(left_value.to_basic_value().into_float_value(), right_value, "add");
                Value::F32 { value }
            }
            _ => return Err(
                CodeGenError::TypeError(
                    format!("{:?}", left_value.get_type()), format!("{:?}", right_value.get_type()),
                )
            ),
        };

        Ok(result_value)
    }

    fn emit_name(&mut self, id: &String) -> Result<Value<'ctx>, CodeGenError> {
        let maybe_symbol = self.symbol_tables.context().get_symbol(id);
        match maybe_symbol {
            // Return the address of the symbol if it exists.
            Some(symbol) => Ok(Value::from_basic_value(
                symbol.value.get_type(),
                self.builder.build_load(symbol.value.get_pointer(), id),
            )),
            // If the symbol does not exist, raise a NameError.
            None => Err(CodeGenError::NameError(id.to_string())),
        }
    }

    fn emit_constant(
        &self,
        value: &ast::Constant,
        _kind: &Option<String>,
    ) -> Result<Value<'ctx>, CodeGenError> {
        match value {
            ast::Constant::None => {
                return Ok(Value::None);
            }
            ast::Constant::Bool(bool) => {
                return Ok(Value::Bool {
                    value: self.context.bool_type().const_int(*bool as u64, false),
                });
            }
            ast::Constant::Str(_) => {}
            ast::Constant::Bytes(_) => {}
            ast::Constant::Int(value) => {
                return Ok(Value::I32 {
                    value: self
                        .context
                        .i32_type()
                        .const_int(truncate_bigint_to_u64(value), true),
                });
            }
            ast::Constant::Tuple(_) => {}
            ast::Constant::Float(value) => {
                return Ok(Value::F32 {
                    value: self.context.f32_type().const_float(*value),
                });
            }
            ast::Constant::Complex { .. } => {}
            ast::Constant::Ellipsis => {}
        }

        Err(CodeGenError::Unimplemented(format!("{:?}", value)))
    }
}
