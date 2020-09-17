use std::error::Error;
use std::fs::{remove_file, write, File};

use clap::{App, Arg, ArgMatches};

use dsp_builder::objcopy;
use dsp_compiler::{get_assembly, CompilerFlags};
use dspython::upload_to;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn parse_arguments<'a>(app: App<'a, '_>) -> ArgMatches<'a> {
    let arg_file = Arg::with_name("file").required(true);
    let arg_port = Arg::with_name("port")
        .long("upload")
        .short("u")
        .takes_value(true);
    let arg_opt = Arg::with_name("opt_level").short("o").takes_value(true);
    let arg_keep_hex = Arg::with_name("keep_hex")
        .long("--keep-hex")
        .takes_value(false);
    let arg_emit_llvm = Arg::with_name("emit_llvm")
        .long("--emit-llvm")
        .takes_value(false);
    let arg_include_libs = Arg::with_name("include_libs")
        .long("--include-libs")
        .short("I")
        .takes_value(true)
        .multiple(true)
        .required(false);

    app.usage(
        r#"usage: dspython [-u PORT] [-o OPT_LEVEL] FILE

positional arguments:
    FILE             Source file

optional arguments:
    -u PORT, --upload PORT
                     Serial Port to upload hex
    -o OPT_LEVEL
                     LLVM Optimization level"#,
    )
    .arg(arg_file)
    .arg(arg_opt)
    .arg(arg_port)
    .arg(arg_keep_hex)
    .arg(arg_emit_llvm)
    .arg(arg_include_libs)
    .get_matches()
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("dspython")
        .version(VERSION)
        .author(AUTHORS)
        .about("DSPython is a damn small Python compiler for Arduino.");

    let matches = parse_arguments(app);

    let file = matches.value_of("file").unwrap();
    let port = matches.value_of("port");

    let optimization_level = matches.value_of("opt_level").unwrap_or("3").parse::<u8>()?;
    let include_libs = if let Some(libs) = matches.values_of("include_libs") {
        libs.collect::<Vec<&str>>()
    } else {
        vec![
            "arduino/constants.py",
            "arduino/uno_pins.py",
            "arduino/math.py",
        ]
    };

    let compiler_flags = CompilerFlags::new(optimization_level, include_libs);

    let assembly = match get_assembly(file.to_string(), compiler_flags) {
        Ok(llvm_string) => llvm_string,
        Err(e) => panic!("{}", e),
    };

    let ll = format!("{}.ll", file);
    {
        write(&ll, assembly.to_string())?;
    }

    let hex = objcopy(&ll);

    if let Some(port) = port {
        upload_to(&hex, port);
    }

    {
        let hex_file = File::open(&hex)?;

        if hex_file.metadata().unwrap().len() > 30 * 1024 {
            println!(
                "WARNING: The size of the result file ({}KB) is larger than 30KB.",
                hex_file.metadata().unwrap().len() / 1024
            );
        }
    }

    // Remove the hex file after finishing upload
    if !matches.is_present("keep_hex") {
        remove_file(hex).unwrap();
    }

    // Remove the llvm ir
    if !matches.is_present("emit_llvm") {
        remove_file(ll).unwrap();
    }

    Ok(())
}
