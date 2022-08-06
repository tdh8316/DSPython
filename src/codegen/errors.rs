use std::fmt;

use inkwell::types::BasicTypeEnum;

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

pub fn get_type_str_from_basic_type(ty: BasicTypeEnum) -> String {
    match ty {
        BasicTypeEnum::ArrayType(_) => "array",
        BasicTypeEnum::FloatType(_) => "float",
        BasicTypeEnum::IntType(_) => "int",
        BasicTypeEnum::PointerType(_) => "pointer",
        BasicTypeEnum::StructType(_) => "struct",
        BasicTypeEnum::VectorType(_) => "vector",
    }.to_string()
}
