use std::error::Error;
use std::fmt;

use dsp_python_parser::ast;

// These errors are not compatible with the parsing errors
#[derive(Debug)]
pub struct LLVMCompileError {
    pub statement: Option<String>,
    pub error: LLVMCompileErrorType,
    pub location: ast::Location,
}

impl LLVMCompileError {}

impl Error for LLVMCompileError {}

impl fmt::Display for LLVMCompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_desc = match &self.error {
            LLVMCompileErrorType::NameError(target) => format!("name '{}' is not defined", target),
        };

        write!(f, "{}: {}", &self.error.to_string(), error_desc)
    }
}

// Errors that can occur during emitting LLVM IR
#[derive(Debug)]
pub enum LLVMCompileErrorType {
    NameError(&'static str),
}

impl fmt::Display for LLVMCompileErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
