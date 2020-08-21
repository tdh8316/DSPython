use std::fmt;

// Errors that can occur during emitting LLVM IR
#[derive(Debug, PartialEq)]
pub enum LLVMCompileErrorType {
    // name '{}' is not defined
    NameError(String),

    SyntaxError(String),

    // Expected '{}', but found '{}'
    TypeError(String, String),

    NotImplemented(String),
}

impl fmt::Display for LLVMCompileErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
