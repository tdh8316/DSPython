use crate::codegen::value::{Value, ValueType};

pub fn get_vargs_types(args_values: Vec<Value>) -> String {
    let mut types_string = String::new();

    for arg in &args_values {
        types_string.push_str(match arg.get_type() {
            ValueType::I32 => "i",
            ValueType::F32 => "f",
            ValueType::Str => "s",
            _ => panic!("Unsupported type"),
        });
    }

    types_string
}
