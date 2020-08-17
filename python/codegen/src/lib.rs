use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;

use dsp_python_parser::ast;

use crate::scope::Globals;
use crate::value::Value;

pub mod scope;
pub mod value;

pub mod cgexpr;
pub mod cgstmt;

pub struct CodeGen<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,

    _fn_value: Option<FunctionValue<'ctx>>,
    pub globals: Globals<'ctx>,
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
            globals: Globals::new(),
        }
    }
}
