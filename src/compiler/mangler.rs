use inkwell::values::BasicMetadataValueEnum;

pub fn get_mangled_func_name(name: &str, args_values: Vec<BasicMetadataValueEnum>) -> String {
    let mut mangled_name = name.to_string();
    mangled_name.push_str("_");

    for arg in &args_values {
        mangled_name.push_str(match arg {
            BasicMetadataValueEnum::ArrayValue(_) => "a",
            BasicMetadataValueEnum::IntValue(_) => "i",
            BasicMetadataValueEnum::FloatValue(_) => "f",
            BasicMetadataValueEnum::PointerValue(_) => "p",
            _ => panic!("Unsupported type"),
        });
    }

    if args_values.len() == 0 {
        mangled_name.push_str("v");
    }

    mangled_name
}
