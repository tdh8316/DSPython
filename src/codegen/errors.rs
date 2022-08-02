use std::fmt;

pub enum CodeGenError {
    CompileError(String),
    NameError(String),
    TypeError(String),
    SyntaxError(String),

    Unimplemented(String),
}

impl fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CodeGenError::CompileError(msg) => write!(f, "CompileError: {}", msg),
            CodeGenError::NameError(msg) => write!(f, "NameError: {}", msg),
            CodeGenError::TypeError(msg) => write!(f, "TypeError: {}", msg),
            CodeGenError::SyntaxError(msg) => write!(f, "SyntaxError: {}", msg),

            CodeGenError::Unimplemented(msg) => write!(f, "Unimplemented: {}", msg),
        }
    }
}
