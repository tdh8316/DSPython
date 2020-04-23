use std::error::Error;

use clap::{App, Arg, ArgMatches, SubCommand};
use inkwell::context::Context;
use inkwell::passes::PassManager;
use inkwell::targets::{TargetData, TargetTriple};

use crate::codegen::compiler::Compiler;

mod codegen;

fn parse_arguments<'a>(app: App<'a, '_>) -> ArgMatches<'a> {
    let version: &'static str = env!("CARGO_PKG_VERSION");
    let authors: &'static str = env!("CARGO_PKG_AUTHORS");
    let app = app
        .version(version)
        .author(authors)
        .about("Damn Small Python is a Python compiler for Arduino")
        .usage("dsp [build|flash] source")
        .subcommand(
            SubCommand::with_name("build")
                .about("Build")
                .arg(Arg::with_name("source").required(true)),
        )
        .subcommand(
            SubCommand::with_name("flash")
                .about("Flash")
                .arg(Arg::with_name("source").required(true))
                .arg(
                    Arg::with_name("port")
                        .required(true)
                        .takes_value(true)
                        .short("p")
                        .long("port"),
                ),
        )
        .arg(Arg::with_name("emit-llvm").long("emit-llvm").global(true));

    app.get_matches()
}

fn build<'a, 'ctx>(source_path: String, emit_llvm: bool) {
    let context = Context::create();
    let target_data = TargetData::create("e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8");
    let module = context.create_module(&source_path.as_str());
    let data_layout = target_data.get_data_layout();

    module.set_data_layout(&data_layout);
    module.set_triple(&TargetTriple::create("avr"));

    let fpm = PassManager::create(&module);
    // fpm.add_instruction_combining_pass();
    fpm.initialize();

    let python_source =
        std::fs::read_to_string(&source_path).expect("Couldn't read the source file");
    let program = rustpython_parser::parser::parse_program(&python_source.as_str()).unwrap();
    let builder = context.create_builder();
    let mut c = Compiler::new(source_path.clone(), &context, &builder, &fpm, &module);

    c.compile_program(program);

    if emit_llvm {
        c.module.print_to_stderr();
    } else {
        c.module
            .print_to_file(source_path + ".ll")
            .expect("Couldn't write file");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("dsp");
    let matches = parse_arguments(app);

    let emit_llvm = matches.is_present("emit-llvm");
    let command = matches.subcommand_name().unwrap();

    if command == "build" {
        build(
            matches
                .subcommand()
                .1
                .unwrap()
                .value_of("source")
                .unwrap()
                .to_string(),
            emit_llvm,
        );
    } else if command == "flash" {
        panic!("NotImplemented")
    }

    Ok(())
}
