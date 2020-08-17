use dsp_compiler_error::*;
use dsp_python_parser::ast;

use crate::CodeGen;
use crate::value::Value;

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
        unimplemented!()
    }

    fn compile_expr_call(
        &mut self,
        func: &Box<ast::Expression>,
        args: &Vec<ast::Expression>,
    ) -> Result<Value<'ctx>, LLVMCompileError> {
        unimplemented!()
    }
}
