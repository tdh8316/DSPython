use std::error::Error;
use std::fmt;

use dsp_python_parser::ast;

pub mod types;
pub use crate::types::LLVMCompileErrorType;

pub mod macros;
pub use crate::macros::*;

// These errors are not compatible with the parsing errors
#[derive(Debug)]
pub struct LLVMCompileError {
    pub error: LLVMCompileErrorType,
    pub location: Option<ast::Location>,
}

impl LLVMCompileError {
    pub fn new(location: Option<ast::Location>, exception: LLVMCompileErrorType) -> Self {
        LLVMCompileError {
            error: exception,
            location,
        }
    }
}

impl Error for LLVMCompileError {}

impl fmt::Display for LLVMCompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(loc) = self.location {
            writeln!(f, "at {:?}:", loc)?;
        }
        let error_desc = match &self.error {
            LLVMCompileErrorType::NameError(target) => format!("name '{}' is not defined", target),
            LLVMCompileErrorType::SyntaxError(desc) => format!("{}", desc),
            LLVMCompileErrorType::TypeError(expected, but) => {
                format!("Expected '{}', but found '{}'", expected, but)
            }
            LLVMCompileErrorType::NotImplemented(desc) => format!("{}", desc),
        };

        write!(f, "{}: {}", &self.error.to_string(), error_desc)
    }
}
