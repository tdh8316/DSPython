use std::error::Error;
use std::fs::read_to_string;
use std::process::Command;

use clap::{App, Arg, ArgMatches};
use inkwell::context::Context;
use inkwell::passes::PassManager;
use rustpython_parser::parser::parse_program;

use crate::compiler::compiler::Compiler;
use crate::ir::ir::new_module;
use std::io::Write;

mod compiler;
mod ir;
mod value;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn parse_arguments<'a>(app: App<'a, '_>) -> ArgMatches<'a> {
    app.arg(Arg::with_name("command").index(1).required(true))
        .arg(Arg::with_name("file").index(2).required(true))
        .arg(Arg::with_name("emit-llvm").long("emit-llvm"))
        .arg(
            Arg::with_name("port")
                .takes_value(true)
                .short("p")
                .long("port"),
        )
        .get_matches()
}

fn build<'a, 'ctx>(pkg: &str, emit_llvm: bool) -> String {
    let ctx = Context::create();
    let module = new_module(pkg, &ctx);

    let pm = PassManager::create(&module);
    pm.initialize();

    let python_source = read_to_string(pkg).unwrap();
    let program = parse_program(&python_source).unwrap();
    let builder = ctx.create_builder();
    let mut c = Compiler::new(String::from(pkg), &ctx, &builder, &pm, &module);

    c.compile(program);
    let assembly = String::from(pkg) + ".ll";
    if emit_llvm {
        c.module.print_to_stderr();
    }
    c.module.print_to_file(&assembly).unwrap();
    {
        assembly
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("dsp")
        .version(VERSION)
        .author(AUTHORS)
        .about("Damn Small Python is a Python compiler for Arduino");

    let matches = parse_arguments(app);

    let command = matches.value_of("command").unwrap();
    let file = matches.value_of("file").unwrap();
    let emit_llvm = matches.is_present("emit-llvm");
    let _arduino_dir = std::env::var("ARDUINO_DIR").expect(
        "You must set the environment variable 'ARDUINO_DIR' as your arduino software location",
    );
    let arduino_dir = _arduino_dir.as_str();

    if command == "build" || command == "flash" || command == "upload" {
        let assembly = build(file, emit_llvm);

        let linker = if cfg!(debug_assertions) {
            "python scripts/linker.py"
        } else {
            "bin/linker"
        };

        let out = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", linker, arduino_dir, assembly.as_str()])
                .status()
                .expect("Failed to execute command")
        } else {
            Command::new("sh")
                .args(&["-c", linker, arduino_dir, assembly.as_str()])
                .status()
                .expect("Failed to execute command")
        };
        if !out.success() {
            panic!("Failed to perform linker.");
        }

        if command == "flash" || command == "upload" {
            let port = matches.value_of("port").expect("Port not provided!");
            print!("{} << {}...", port, file);
            std::io::stdout().flush().unwrap_or_default();

            let uploader = if cfg!(debug_assertions) {
                "python scripts/flash.py"
            } else {
                "bin/flash"
            };

            let out = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(&[
                        "/C",
                        uploader,
                        arduino_dir,
                        &(assembly.to_owned() + ".hex"),
                        port,
                    ])
                    .output()
                    .expect("Failed to execute command")
            } else {
                Command::new("sh")
                    .args(&[
                        "-c",
                        uploader,
                        arduino_dir,
                        &(assembly.to_owned() + ".hex"),
                        port,
                    ])
                    .output()
                    .expect("Failed to execute command")
            };

            if !out.status.success() {
                println!("{}", String::from_utf8_lossy(&out.stderr));
                panic!("Failed to perform uploading.");
            } else {
                println!("[Done]")
            }
        }
    } else {
        panic!(format!("Unknown command '{}'", command));
    }

    Ok(())
}
