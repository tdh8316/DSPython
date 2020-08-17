use std::error::Error;
use std::fs::write;

use clap::{App, Arg, ArgMatches};

use dsp_compiler::{compile, CompilerFlags};
use dspython::upload_to;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("dspython")
        .version(VERSION)
        .author(AUTHORS)
        .about("DSPython is a damn small Python compiler for Arduino.");

    let matches = parse_arguments(app);

    let file = matches.value_of("file").unwrap();
    let port = matches.value_of("port");

    let opt_level = matches
        .value_of("opt_level")
        .unwrap_or("3")
        .parse::<u8>()
        .unwrap();

    let compiler_flags = CompilerFlags::new(opt_level);

    let assembly = compile(file.to_string(), compiler_flags)?;

    let ll = format!("{}.ll", file);
    write(&ll, assembly.to_string()).unwrap();

    if let Some(port) = port {
        upload_to(&ll, port);
    }

    Ok(())
}

fn parse_arguments<'a>(app: App<'a, '_>) -> ArgMatches<'a> {
    let arg_file = Arg::with_name("file").required(true);
    let arg_port = Arg::with_name("port")
        .long("upload")
        .short("u")
        .takes_value(true);
    let arg_opt = Arg::with_name("opt_level").short("O").takes_value(true);

    app.usage(
        r#"usage: dspython [-u PORT] FILE

positional arguments:
    FILE             Source file

optional arguments:
    -u PORT, --upload PORT
                     Serial Port to upload hex"#,
    )
        .arg(arg_file)
        .arg(arg_opt)
        .arg(arg_port)
        .get_matches()
}
