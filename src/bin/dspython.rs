use std::error::Error;
use std::fs::{remove_file, write, File};

use clap::{App, Arg, ArgMatches};

use dsp_builder::objcopy;
use dsp_compiler::{get_assembly, CompilerFlags};
use dspython::avrdude;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn parse_arguments<'a>(app: App<'a, '_>) -> ArgMatches<'a> {
    let arg_file = Arg::with_name("file")
        .required(true)
        .help("The source file");
    let arg_port = Arg::with_name("port")
        .help("Serial port of an Arduino to upload")
        .long("--upload-to")
        .short("u")
        .takes_value(true);
    let arg_opt = Arg::with_name("opt_level")
        .help("LLVM Optimization level. Must be in the range of 0 to 3")
        .long("--opt-level")
        .short("o")
        .takes_value(true);
    let arg_baudrate = Arg::with_name("baudrate")
        .help("Serial communication speed")
        .long("--baudrate")
        .short("b")
        .takes_value(true);
    let arg_remove_hex = Arg::with_name("remove_hex")
        .help("Remove generated hex file")
        .long("--remove-hex")
        .takes_value(false);
    let arg_emit_llvm = Arg::with_name("emit_llvm")
        .help("Emit LLVM IR")
        .long("--emit-llvm")
        .takes_value(false);

    app.arg(arg_file)
        .arg(arg_opt)
        .arg(arg_baudrate)
        .arg(arg_port)
        .arg(arg_remove_hex)
        .arg(arg_emit_llvm)
        .get_matches()
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("dspython")
        .version(VERSION)
        .author(AUTHORS)
        .about("DSPython is a damn small Python compiler intended to use in Arduino.");

    let matches = parse_arguments(app);

    let file = matches.value_of("file").expect("no input files");
    let port = matches.value_of("port");

    let optimization_level = matches.value_of("opt_level").unwrap_or("2").parse::<u8>()?;
    let baudrate = matches.value_of("baudrate").unwrap_or("9600").parse::<u64>()?;

    let compiler_flags = CompilerFlags::new(optimization_level);

    let assembly = match get_assembly(file.to_string(), compiler_flags) {
        Ok(llvm_string) => llvm_string,
        Err(e) => panic!("{}", e),
    };

    let ir_path = format!("{}.ll", file);
    {
        write(&ir_path, assembly.to_string())?;
    }

    let hex = objcopy(&ir_path);
    {
        let hex_file = File::open(&hex)?;
        let file_size = hex_file.metadata().unwrap().len();

        /*
        println!(
            "The result is written to {} ({}KB of 30KB)",
            &hex,
            file_size / 1024
        );
        */

        if file_size > 30 * 1024 {
            eprintln!(
                "WARNING: The size of the result file ({}KB) is larger than 30KB.",
                file_size / 1024
            );
        }
    }

    if let Some(port) = port {
        avrdude(&hex, port, baudrate);
    }

    // Remove the hex file if --remove-hex is presented after finishing upload
    if matches.is_present("remove_hex") {
        remove_file(hex).unwrap();
    }

    // Remove the llvm ir if --emit-llvm is not presented
    if !matches.is_present("emit_llvm") {
        remove_file(&ir_path).unwrap();
    }
    
    // Remove object files
    remove_file(ir_path.clone() + ".eep").unwrap();
    remove_file(ir_path.clone() + ".elf").unwrap();
    remove_file(ir_path.clone() + ".o").unwrap();

    Ok(())
}
