use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use rustpython_parser::ast;

use crate::codegen::errors::CodeGenError;
use crate::codegen::symbol_table::{SymbolTable, SymbolTables};

mod cgexpr;
mod cgstmt;
mod errors;
mod symbol_table;
mod value;

pub struct CodeGenArgs {}

/// Main structure that holding the state of code generation
pub struct CodeGen<'a, 'ctx> {
    context: &'ctx Context,
    module: &'a Module<'ctx>,
    builder: &'a Builder<'ctx>,
    symbol_tables: SymbolTables<'ctx>,
    global_symbol_table: SymbolTable<'ctx>,
    current_source_location: ast::Location,
    args: CodeGenArgs,
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        builder: &'a Builder<'ctx>,
        args: CodeGenArgs,
    ) -> Self {
        Self {
            context,
            module,
            builder,
            symbol_tables: SymbolTables::new("__main__".to_string()),
            global_symbol_table: SymbolTable::new("__globals__".to_string()),
            current_source_location: ast::Location::default(),
            args,
        }
    }

    fn set_source_location(&mut self, location: ast::Location) {
        self.current_source_location = location;
    }

    pub fn get_source_location(&self) -> ast::Location {
        self.current_source_location
    }

    pub fn emit(&self) -> String {
        self.module.print_to_string().to_string()
    }
}
