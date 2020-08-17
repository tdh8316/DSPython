use std::collections::HashMap;

use inkwell::values::PointerValue;

use crate::value::ValueType;

pub struct Globals<'ctx> {
    variables: HashMap<String, (ValueType, PointerValue<'ctx>)>,
}

impl<'ctx> Globals<'ctx> {
    pub fn new() -> Self {
        Globals {
            variables: HashMap::new(),
        }
    }
}
