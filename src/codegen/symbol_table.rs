use std::collections::HashMap;

use inkwell::values::PointerValue;

use crate::codegen::value::ValueType;

pub struct SymbolTables<'ctx> {
    symbol_tables: HashMap<String, SymbolTable<'ctx>>,
    current_namespace: String,
}

impl<'ctx> SymbolTables<'ctx> {
    pub fn new(current_namespace: String) -> Self {
        Self {
            symbol_tables: HashMap::new(),
            current_namespace,
        }
    }

    /// Get or create the symbol table of the given namespace
    fn get_or_create(&mut self, name: String) -> &mut SymbolTable<'ctx> {
        if !self.symbol_tables.contains_key(&name) {
            self.symbol_tables
                .insert(name.clone(), SymbolTable::new(name.clone()));
        }
        self.symbol_tables.get_mut(&name).unwrap()
    }

    /// Set the current namespace and return a new symbol table for the new namespace
    fn set_namespace(&mut self, name: String) -> &mut SymbolTable<'ctx> {
        self.current_namespace = name.clone();
        self.get_or_create(name)
    }

    /// Get the symbol table of the current namespace context
    pub fn context(&mut self) -> &mut SymbolTable<'ctx> {
        self.get_or_create(self.current_namespace.clone())
    }

    /// Push a new namespace and return a new symbol table for the new namespace
    pub fn push_namespace(&mut self, name: String) -> &mut SymbolTable<'ctx> {
        let new_namespace = format!("{}::{}", self.current_namespace, name);
        self.set_namespace(new_namespace)
    }

    /// Pop the current namespace
    pub fn pop_namespace(&mut self) {
        let parts: Vec<&str> = self.current_namespace.split("::").collect();
        if parts.len() > 1 {
            self.current_namespace = parts[..parts.len() - 1].join("::");
        } else {
            self.current_namespace = "".to_string();
        }
    }
}

pub struct SymbolTable<'ctx> {
    pub name: String,
    symbols: HashMap<String, Symbol<'ctx>>,
}

impl<'ctx> SymbolTable<'ctx> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            symbols: HashMap::new(),
        }
    }

    /// Add a new symbol to the symbol table
    /// Update the symbol if the symbol already exists
    pub fn add_symbol(&mut self, symbol: Symbol<'ctx>) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol<'ctx>> {
        self.symbols.get(name)
    }
}

#[derive(PartialEq)]
pub enum SymbolScope {
    Global,
    Local,
}

pub type SymbolValue<'ctx> = (ValueType, PointerValue<'ctx>);

pub trait SymbolValueTrait<'ctx> {
    fn get_type(&self) -> ValueType;
    fn get_pointer(&self) -> PointerValue<'ctx>;
}

impl<'ctx> SymbolValueTrait<'ctx> for SymbolValue<'ctx> {
    fn get_type(&self) -> ValueType {
        self.0
    }
    fn get_pointer(&self) -> PointerValue<'ctx> {
        self.1
    }
}

pub struct Symbol<'ctx> {
    pub name: String,
    pub value: SymbolValue<'ctx>,
    #[allow(unused)]
    scope: SymbolScope,
}

impl<'ctx> Symbol<'ctx> {
    pub fn new(name: String, value: SymbolValue<'ctx>, scope: SymbolScope) -> Self {
        Self { name, value, scope }
    }

    #[allow(unused)]
    pub fn is_global(&self) -> bool {
        self.scope == SymbolScope::Global
    }
}
