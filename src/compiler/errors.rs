use std::fmt;

pub enum CompilerError {
    LLVMError(String),
    ParseError(String),
    CodeGenError(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilerError::LLVMError(msg) => write!(f, "LLVMError: {}", msg),
            CompilerError::ParseError(msg) => write!(f, "ParseError: {}", msg),
            CompilerError::CodeGenError(msg) => write!(f, "{}", msg),
        }
    }
}
