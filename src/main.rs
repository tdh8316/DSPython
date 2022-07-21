use clap::Parser;

use dspython::compiler::Compiler;

#[derive(Parser)]
#[clap(long_about = None)]
struct Args {
    file_name: String,

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

    let compiler = Compiler::new(args.opt_level, args.size_level);

    let _output_path = compiler.compile_file(args.file_name.as_str());
}
