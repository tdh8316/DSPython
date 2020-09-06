use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;

use dsp_compiler_error::{err, LLVMCompileError, LLVMCompileErrorType};
use dsp_python_parser::ast;

use crate::scope::{Locals, VariableMap};

pub mod scope;

pub mod cgexpr;
pub mod cgstmt;

pub struct CompileContext {
    returned: bool,
}

pub struct CodeGen<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,

    _fn_value: Option<FunctionValue<'ctx>>,
    _current_source_location: ast::Location,
    globals: VariableMap<'ctx>,
    locals: Locals<'ctx>,
    compile_context: CompileContext,
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
    ) -> Self {
        CodeGen {
            context,
            builder,
            module,
            _fn_value: None,
            _current_source_location: ast::Location::default(),
            globals: VariableMap::new(),
            locals: Locals::new(),
            compile_context: CompileContext { returned: false },
        }
    }

    #[inline]
    pub fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(name)
    }

    pub fn set_fn_value(&mut self, fn_value: FunctionValue<'ctx>) {
        self._fn_value = Some(fn_value);
    }

    pub fn get_fn_value(&self) -> Result<FunctionValue<'ctx>, LLVMCompileError> {
        match self._fn_value {
            Some(func) => Ok(func),
            None => err!(
                self,
                LLVMCompileErrorType::NotImplemented,
                "Attempted to get a function value outside function. Some features must be in the function."
            )
        }
    }

    fn set_loc(&mut self, location: ast::Location) {
        self._current_source_location = location;
    }

    pub fn get_loc(&self) -> ast::Location {
        self._current_source_location
    }
}
