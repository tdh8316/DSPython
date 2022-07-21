use rustpython_parser::ast;

use crate::codegen::errors::CodeGenError;
use crate::codegen::symbol_table::SymbolValueTrait;
use crate::codegen::value::{truncate_bigint_to_u64, Value};
use crate::codegen::CodeGen;

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    pub fn emit_expr(&mut self, expr: &ast::Expr) -> Result<Value<'ctx>, CodeGenError> {
        self.set_source_location(expr.location);
        use ast::ExprKind::*;
        match &expr.node {
            Constant { value, kind } => self.emit_constant(value, kind),
            Name { id, .. } => self.emit_name(id),

            _ => Err(CodeGenError::Unimplemented(format!("expr: {:?}", expr))),
        }
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
