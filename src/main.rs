use clap::Parser;

use dspython::compiler::Compiler;

#[derive(Parser)]
#[clap(long_about = None)]
struct Args {
    file_name: Option<String>,

    #[clap(
        short,
        long,
        value_parser,
        default_value = "3",
        help = "Optimization level"
    )]
    opt_level: u32,
    #[clap(short, long, value_parser, default_value = "2", help = "Size level")]
    size_level: u32,
}

fn main() {
    let args: Args = Args::parse();

    if let Some(file_name) = args.file_name {
        let compiler = Compiler::new(args.opt_level, args.size_level);
        let _ir_path = match compiler.compile_file(file_name.as_str()) {
            Ok(ir_path) => ir_path,
            Err(err) => {
                panic!("{}", err);
            }
        };
    } else {
        panic!("No input file");
    }
}
