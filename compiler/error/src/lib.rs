use std::error::Error;
use std::fmt;

use dsp_python_parser::ast;

// These errors are not compatible with the parsing errors
#[derive(Debug)]
pub struct LLVMCompileError {
    pub error: LLVMCompileErrorType,
    pub location: ast::Location,
}

impl LLVMCompileError {
    pub fn new(location: ast::Location, exception: LLVMCompileErrorType) -> Self {
        LLVMCompileError {
            error: exception,
            location,
        }
    }
}

impl Error for LLVMCompileError {}

impl fmt::Display for LLVMCompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_desc = match &self.error {
            LLVMCompileErrorType::NameError(target) => format!("name '{}' is not defined", target),
            LLVMCompileErrorType::SyntaxError(desc) => format!("{}", desc),
            LLVMCompileErrorType::TypeError(expected, but) => {
                format!("Expected '{}', but found '{}'", expected, but)
            }
            LLVMCompileErrorType::NotImplemented(desc) => {
                desc.as_ref().unwrap_or(&String::new()).to_string()
            }
        };

        write!(f, "{}: {}", &self.error.to_string(), error_desc)
    }
}

// Errors that can occur during emitting LLVM IR
#[derive(Debug)]
pub enum LLVMCompileErrorType {
    NameError(String),
    SyntaxError(String),
    TypeError(String, String),

    NotImplemented(Option<String>),
}

impl fmt::Display for LLVMCompileErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
