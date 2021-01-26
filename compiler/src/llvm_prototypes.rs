use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::AddressSpace;

// TODO: Generate prototypes using included files
pub fn generate_prototypes<'a, 'ctx>(module: &'a Module<'ctx>, context: &'ctx Context) {
    // Arduino builtins
    module.add_function(
        "pin_mode",
        context
            .void_type()
            .fn_type(&[context.i8_type().into(), context.i8_type().into()], false),
        None,
    );
    module.add_function(
        "pulse_in",
        context
            .f32_type()
            .fn_type(&[context.i8_type().into(), context.i8_type().into()], false),
        None,
    );
    module.add_function(
        "is_serial_available",
        context.bool_type().fn_type(&[], false),
        None,
    );
    module.add_function(
        "serial_begin",
        context
            .void_type()
            .fn_type(&[context.i16_type().into()], false),
        None,
    );
    module.add_function("input", context.i16_type().fn_type(&[], false), None);
    module.add_function("flush", context.void_type().fn_type(&[], false), None);
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
    module.add_function(
        "analog_write",
        context
            .void_type()
            .fn_type(&[context.i8_type().into(), context.i8_type().into()], false),
        None,
    );
    module.add_function(
        "digital_read",
        context
            .i16_type()
            .fn_type(&[context.i8_type().into()], false),
        None,
    );
    module.add_function(
        "analog_read",
        context
            .i16_type()
            .fn_type(&[context.i8_type().into()], false),
        None,
    );

    // Math
    module.add_function(
        "sin",
        context
            .f32_type()
            .fn_type(&[context.f32_type().into()], false),
        None,
    );
    module.add_function(
        "cos",
        context
            .f32_type()
            .fn_type(&[context.f32_type().into()], false),
        None,
    );
    module.add_function(
        "tan",
        context
            .f32_type()
            .fn_type(&[context.f32_type().into()], false),
        None,
    );

    // Python builtins
    module.add_function(
        "print__i__",
        context
            .void_type()
            .fn_type(&[context.i16_type().into()], false),
        None,
    );
    module.add_function(
        "print__f__",
        context
            .void_type()
            .fn_type(&[context.f32_type().into()], false),
        None,
    );
    module.add_function(
        "print__s__",
        context.void_type().fn_type(
            &[context.i8_type().ptr_type(AddressSpace::Generic).into()],
            false,
        ),
        None,
    );
    module.add_function(
        "println__i__",
        context
            .void_type()
            .fn_type(&[context.i16_type().into()], false),
        None,
    );
    module.add_function(
        "println__f__",
        context
            .void_type()
            .fn_type(&[context.f32_type().into()], false),
        None,
    );
    module.add_function(
        "println__s__",
        context.void_type().fn_type(
            &[context.i8_type().ptr_type(AddressSpace::Generic).into()],
            false,
        ),
        None,
    );
    module.add_function(
        "int__f__",
        context
            .i16_type()
            .fn_type(&[context.f32_type().into()], false),
        None,
    );
    module.add_function(
        "int__i__",
        context
            .i16_type()
            .fn_type(&[context.i16_type().into()], false),
        None,
    );
    module.add_function(
        "float__f__",
        context
            .f32_type()
            .fn_type(&[context.f32_type().into()], false),
        None,
    );
    module.add_function(
        "float__i__",
        context
            .f32_type()
            .fn_type(&[context.i16_type().into()], false),
        None,
    );
}
