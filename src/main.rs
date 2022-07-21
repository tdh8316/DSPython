use clap::Parser;

use dspython::compiler::Compiler;

#[derive(Parser)]
#[clap(long_about = None)]
struct Args {
    file_name: String,
}

fn main() {
    let args: Args = Args::parse();

    let compiler = Compiler::new();

    let output_path = compiler.compile_file(args.file_name.as_str());
}
