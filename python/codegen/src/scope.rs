use std::collections::HashMap;

use inkwell::values::{FunctionValue, PointerValue};

use dsp_compiler_value::value::ValueType;

/*
Fuck you inappropriate lifetime error
                         _
 _._ _..._ .-',     _.._(`))
'-. `     '  /-._.-'    ',/
   )         \            '.
  / _    _    |             \
 |  a    a    /              |
 \   .-.                     ;
  '-('' ).-'       ,'       ;
     '-;           |      .'
        \           \    /
        | 7  .__  _.-\   \
        | |  |  ``/  /`  /
       /,_|  |   /,_/   /
          /,_/      '`-'
 */

pub type LLVMVariable<'ctx> = (ValueType, PointerValue<'ctx>);

pub trait LLVMVariableAccessor<'ctx> {
    fn value_type(&self) -> ValueType;
    fn pointer_value(&self) -> PointerValue<'ctx>;
}

impl<'ctx> LLVMVariableAccessor<'ctx> for LLVMVariable<'ctx> {
    fn value_type(&self) -> ValueType {
        self.0
    }

    fn pointer_value(&self) -> PointerValue<'ctx> {
        self.1
    }
}

pub struct VariableMap<'ctx> {
    variables: HashMap<String, LLVMVariable<'ctx>>,
}

impl<'ctx> VariableMap<'ctx> {
    pub fn new() -> Self {
        VariableMap {
            variables: HashMap::new(),
        }
    }

    pub fn load(&self, name: &str) -> Option<&LLVMVariable<'ctx>> {
        self.variables.get(name)
    }

    pub fn set(&mut self, name: &str, value: LLVMVariable<'ctx>) {
        self.variables.insert(name.to_string(), value);
    }
}

pub struct Locals<'ctx> {
    local_variables: HashMap<FunctionValue<'ctx>, VariableMap<'ctx>>,
}

impl<'ctx> Locals<'ctx> {
    pub fn new() -> Self {
        Locals {
            local_variables: HashMap::new(),
        }
    }

    pub fn load(&self, fn_value: &FunctionValue<'ctx>, name: &str) -> Option<&LLVMVariable<'ctx>> {
        self.local_variables.get(fn_value).unwrap().load(name)
    }

    pub fn set(&mut self, fn_value: &FunctionValue<'ctx>, name: &str, value: LLVMVariable<'ctx>) {
        self.local_variables
            .get_mut(fn_value)
            .unwrap()
            .set(name, value);
    }

    pub fn create(&mut self, fn_value: FunctionValue<'ctx>) {
        self.local_variables.insert(fn_value, VariableMap::new());
    }
}
