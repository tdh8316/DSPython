use std::fmt;

pub enum CodeGenError {
    NameError(String),
    TypeError(String, String),

    Unimplemented(String),
    CompileError(String),
}

impl fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CodeGenError::NameError(name) => write!(f, "NameError: name '{}' is not defined", name),
            CodeGenError::TypeError(expected, but) => {
                write!(f, "TypeError: Expected '{}', but found '{}'", expected, but)
            }

            CodeGenError::Unimplemented(msg) => write!(f, "Unimplemented: {}", msg),
            CodeGenError::CompileError(msg) => write!(f, "CompileError: {}", msg),
        }
    }
}
