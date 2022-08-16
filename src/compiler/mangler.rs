use crate::codegen::value::{Value, ValueType};

pub fn get_mangled_func_name(name: &str, args_values: &Vec<Value>) -> String {
    let mut mangled_name = name.to_string();
    mangled_name.push_str("_");

    for arg in args_values {
        mangled_name.push_str(match arg.get_type() {
            ValueType::I32 => "i",
            ValueType::F32 => "f",
            ValueType::Str => "s",
            _ => panic!("Unsupported type"),
        });
        // Only the first argument is used for the mangled name
        break;
    }

    if args_values.len() == 0 {
        mangled_name.push_str("v");
    }

    mangled_name
}
