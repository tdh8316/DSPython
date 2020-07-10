use crate::compiler::Compiler;

#[derive(Debug)]
pub enum CompilerErrorType<'a> {
    NameError(&'a str),
    // ValueError(&'a str),
}

pub trait CompilerErrorReport<'a, 'ctx> {
    fn errs(&mut self, msg: CompilerErrorType) -> String;
}

impl<'a, 'ctx> CompilerErrorReport<'a, 'ctx> for Compiler<'a, 'ctx> {
    fn errs(&mut self, err: CompilerErrorType) -> String {
        let mut desc = String::new();
        desc.push_str("Traceback (most recent call last):\n");
        // TODO: Error scope
        desc.push_str(
            format!(
                "  File \"{}\", at {:?}, in \"Unknown\"\n",
                self.source_path.as_str(),
                self.current_source_location
            )
            .as_ref(),
        );
        let err_desc = match err {
            CompilerErrorType::NameError(s) => format!("name '{}' is not defined.\n", s),
            // CompilerErrorType::ValueError(s) => s.to_owned(),
        };

        desc.push_str(err_desc.as_ref());

        desc
    }
}
