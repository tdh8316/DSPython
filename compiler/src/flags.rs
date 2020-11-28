#[derive(Clone)]
pub struct CompilerFlags {
    pub optimization_level: u8,
}

impl CompilerFlags {
    pub fn new(optimization_level: u8) -> Self {
        CompilerFlags { optimization_level }
    }
}
