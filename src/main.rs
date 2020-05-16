use std::error::Error;
use std::fs::{read_to_string, remove_file};
use std::io::Write;
use std::process::Command;

use clap::{App, Arg, ArgMatches};
use inkwell::context::Context;
use inkwell::passes::PassManager;
use inkwell::targets::{TargetData, TargetTriple};
use rustpython_parser::parser::parse_program;

mod compiler;
mod irgen;
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

fn build<'a, 'ctx>(pkg: &str) -> String {
    let ctx = Context::create();
    let module = ctx.create_module(pkg);

    // Create target data structure for Arduino
    let target_data = TargetData::create("e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8");
    module.set_data_layout(&target_data.get_data_layout());

    // LLVM triple
    module.set_triple(&TargetTriple::create("avr"));

    let pm = PassManager::create(&module);
    pm.initialize();

    // Read source code from package file
    let python_source = read_to_string(pkg).unwrap();

    // Parse the source code
    let program = parse_program(&python_source).unwrap();

    // Create a root builder context
    let builder = ctx.create_builder();

    let mut c = compiler::Compiler::new(String::from(pkg), &ctx, &builder, &pm, &module);
    c.compile(program);

    // LLVM assembly path
    let assembly = String::from(pkg) + ".ll";

    // Write assembly to file
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

    // Parse command-line arguments
    let matches = parse_arguments(app);

    let command = matches.value_of("command").unwrap();
    let file = matches.value_of("file").unwrap();
    let emit_llvm = matches.is_present("emit-llvm");

    // Load the environmental variable: `ARDUINO_DIR`
    let _arduino_dir = std::env::var("ARDUINO_DIR").expect(
        "You must set the environment variable 'ARDUINO_DIR' as your arduino software location",
    );
    let arduino_dir = _arduino_dir.as_str();

    // Check if command is valid
    if !(command == "build" || command == "flash" || command == "upload") {
        panic!(format!("Unknown command '{}'", command));
    }

    // Build assembly from python file and return it
    let assembly = build(file);

    // Linker path
    let linker = if cfg!(debug_assertions) {
        "python builder/linker.py"
    } else {
        "bin/linker"
    };

    // Execute the linker command
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

        // Uploader path
        let uploader = if cfg!(debug_assertions) {
            "python scripts/flash.py"
        } else {
            "bin/flash"
        };

        // Hex file path
        let hex_file = &(assembly.to_owned() + ".hex");

        // Execute the uploader command
        let out = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", uploader, arduino_dir, hex_file, port])
                .output()
                .expect("Failed to execute command")
        } else {
            Command::new("sh")
                .args(&["-c", uploader, arduino_dir, hex_file, port])
                .output()
                .expect("Failed to execute command")
        };
        if !out.status.success() {
            println!("{}", String::from_utf8_lossy(&out.stderr));
            panic!("Failed to perform uploading.");
        } else {
            // Remove the hex file
            remove_file(hex_file).unwrap();
            println!("[Done]")
        }
    }

    // Remove the assembly file if flag `emit-llvm` is not enabled
    if !emit_llvm {
        remove_file(&assembly).unwrap();
    }

    Ok(())
}
