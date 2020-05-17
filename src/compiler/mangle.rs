use crate::value::ValueType;

pub fn mangling(origin: &mut String, at: ValueType) -> &String {
    origin.push_str(match at {
        ValueType::Str => "__s__",
        ValueType::I16 => "__i__",
        ValueType::F32 => "__f__",
        _ => "",
    });

    {
        origin
    }
}
