use dsp_compiler_value::value::ValueType;

pub fn mangling(origin: &String, at: ValueType) -> String {
    let origin = &mut origin.clone();

    origin.push_str(match at {
        ValueType::Str => "__s__",
        ValueType::I8 | ValueType::I16 | ValueType::Bool => "__i__",
        ValueType::F32 => "__f__",
        _ => "",
    });

    {
        origin.clone()
    }
}
