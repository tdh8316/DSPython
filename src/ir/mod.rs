#[cfg(test)]
mod tests {
    use inkwell::builder::Builder;
    use inkwell::context::Context;
    use inkwell::module::Module;
    use inkwell::targets::{TargetData, TargetTriple};
    use std::path::Path;
    use std::fs::File;
    use std::io::Write;

    #[derive(Debug)]
    struct CodeGen<'ctx> {
        context: &'ctx Context,
        module: Module<'ctx>,
        builder: Builder<'ctx>,
    }

    impl<'ctx> CodeGen<'ctx> {
        fn led(&self) {
            let void = self.context.void_type();
            let i8 = self.context.i8_type();

            let t_func_setup = void.fn_type(&[], false);
            let func_setup = self.module.add_function("setup", t_func_setup, None);
            let b_func_setup = self.context.append_basic_block(func_setup, "");

            self.builder.position_at_end(b_func_setup);

            let t_func_pin_mode = void.fn_type(&[i8.into(), i8.into()], false);
            let func_pin_mode = self.module.add_function("pinMode", t_func_pin_mode, None);
            let args_pin_mode = [
                i8.const_int(13, false).into(),
                i8.const_int(1, false).into(),
            ];
            let call_pin_mode = self
                .builder
                .build_call(func_pin_mode, &args_pin_mode, "pinMode");
            call_pin_mode.set_tail_call(true);

            self.builder.build_return(None);

            let t_func_loop = void.fn_type(&[], false);
            let func_loop = self.module.add_function("loop", t_func_loop, None);
            let b_func_loop = self.context.append_basic_block(func_loop, "");

            self.builder.position_at_end(b_func_loop);

            let t_func_digital_write = void.fn_type(&[i8.into(), i8.into()], false);
            let func_digital_write =
                self.module
                    .add_function("digitalWrite", t_func_digital_write, None);
            let args_digital_write = [
                i8.const_int(13, false).into(),
                i8.const_int(1, false).into(),
            ];
            let call_digital_write =
                self.builder
                    .build_call(func_digital_write, &args_digital_write, "digitalWrite");
            call_digital_write.set_tail_call(true);

            self.builder.build_return(None);
        }
    }

    #[test]
    fn generate_test_led() {
        let target_data =
            TargetData::create("e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8");
        let context = Context::create();
        let module = context.create_module("LED.ino");
        let data_layout = target_data.get_data_layout();

        module.set_data_layout(&data_layout);
        module.set_triple(&TargetTriple::create("avr"));

        let code_gen = CodeGen {
            context: &context,
            module,
            builder: context.create_builder(),
        };
        code_gen.led();

        let path = Path::new("turn_on_led_test.ll");
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", path.display(), why.to_string()),
            Ok(file) => file,
        };

        match file.write_all(code_gen.module.print_to_string().to_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", path.display(), why.to_string()),
            Ok(_) => println!("LLVM IR is generated to {}", path.display()),
        }
    }
}
