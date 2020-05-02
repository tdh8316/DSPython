use crate::value::value::ValueType;

pub fn mangling(origin: &mut String, at: ValueType) -> &String {
    println!("{:?}", at);
    origin.push_str(match at {
        ValueType::Str => "s",
        ValueType::I16 => "i",
        ValueType::F32 => "f",
        _ => "",
    });

    {
        origin
    }
}
