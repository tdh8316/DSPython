use dsp_compiler_error::*;
use dsp_python_parser::ast;

use crate::CodeGen;
use crate::value::Value;

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
        unimplemented!()
    }

    fn compile_stmt_function_def(
        &mut self,
        name: &String,
        args: &Box<ast::Parameters>,
        body: &ast::Suite,
        returns: &Option<ast::Expression>,
    ) -> Result<(), LLVMCompileError> {
        unimplemented!()
    }

    fn compile_stmt_conditional(
        &mut self,
        cond: Value<'ctx>,
        body: &Vec<ast::Statement>,
        orelse: Option<&Vec<ast::Statement>>,
    ) -> Result<(), LLVMCompileError> {
        unimplemented!()
    }

    fn compile_stmt_while(
        &mut self,
        test: &ast::Expression,
        body: &ast::Suite,
        orelse: &Option<ast::Suite>,
    ) -> Result<(), LLVMCompileError> {
        unimplemented!()
    }
}
