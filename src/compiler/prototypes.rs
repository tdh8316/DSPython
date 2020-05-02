use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::AddressSpace;

pub fn generate_prototypes<'a, 'ctx>(module: &'a Module<'ctx>, context: &'ctx Context) {
    module.add_function(
        "pin_mode",
        context
            .void_type()
            .fn_type(&[context.i8_type().into(), context.i8_type().into()], false),
        None,
    );
    module.add_function(
        "begin",
        context
            .void_type()
            .fn_type(&[context.i16_type().into()], false),
        None,
    );
    module.add_function(
        "printi",
        context
            .void_type()
            .fn_type(&[context.i16_type().into()], false),
        None,
    );
    module.add_function(
        "printf",
        context
            .void_type()
            .fn_type(&[context.f32_type().into()], false),
        None,
    );
    module.add_function(
        "prints",
        context.void_type().fn_type(
            &[context.i8_type().ptr_type(AddressSpace::Generic).into()],
            false,
        ),
        None,
    );
    module.add_function(
        "delay",
        context
            .void_type()
            .fn_type(&[context.i32_type().into()], false),
        None,
    );
    module.add_function(
        "digital_write",
        context
            .void_type()
            .fn_type(&[context.i8_type().into(), context.i8_type().into()], false),
        None,
    );
}
