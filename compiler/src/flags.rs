#[derive(Clone)]
pub struct CompilerFlags<'a> {
    pub optimization_level: u8,
    pub include_libs: Vec<&'a str>,
}

impl<'a> CompilerFlags<'a> {
    pub fn new(optimization_level: u8, include_libs: Vec<&'a str>) -> Self {
        CompilerFlags {
            optimization_level,
            include_libs,
        }
    }
}
