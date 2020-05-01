use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{TargetData, TargetTriple};

pub(crate) fn new_module<'ctx>(name: &str, context: &'ctx Context) -> Module<'ctx> {
    let target_data = TargetData::create("e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8");
    let module = context.create_module(name);

    module.set_data_layout(&target_data.get_data_layout());
    // module.set_triple(&TargetTriple::create("avr-atmel-none"));
    module.set_triple(&TargetTriple::create("avr"));

    {
        module
    }
}
