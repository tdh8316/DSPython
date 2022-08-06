use inkwell::context::Context;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::module::Module;

use crate::clang::Clang;
use crate::compiler::errors::CompilerError;

pub struct ModuleLinker<'a, 'ctx> {
    context: &'ctx Context,
    module: &'a Module<'ctx>,
}

impl<'a, 'ctx> ModuleLinker<'a, 'ctx> {
    pub fn new(context: &'ctx Context, module: &'a Module<'ctx>) -> Self {
        Self { context, module }
    }

    pub fn include_core(&mut self) -> Result<(), CompilerError> {
        let clang = Clang::new();
        let headers = std::fs::read_dir("python/src/").unwrap();
        for header in headers {
            let header = header.unwrap();
            if let Err(e) = clang.run(&["-S", "-emit-llvm", "-O3", header.path().to_str().unwrap(),
                "-o", format!("build/{}.ll", header.file_name().to_str().unwrap()).as_str()]) {
                return Err(CompilerError::LLVMError(e));
            }
        }

        for ir in std::fs::read_dir("build/").unwrap() {
            let ir = ir.unwrap();
            let buf = MemoryBuffer::create_from_file(ir.path().as_path()).unwrap();
            self.module.link_in_module(self.context.create_module_from_ir(buf).unwrap()).unwrap();
        }

        Ok(())
    }
}