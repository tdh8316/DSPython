use std::process::Command;

use clap::Parser;

use dspython::clang::Clang;
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
        let ir_path = match compiler.compile_file(file_name.as_str()) {
            Ok(ir_path) => ir_path,
            Err(err) => {
                panic!("{}", err);
            }
        };

        // TODO: Add platform-specific compilation
        let clang = Clang::new();
        clang
            .run(&[ir_path.as_str(), "-o", format!("{}.exe", ir_path).as_str()])
            .unwrap();
        let out = Command::new(format!("{}.exe", ir_path).as_str())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();
        println!("{}", String::from_utf8_lossy(&out.stdout));
    } else {
        panic!("No input file");
    }
}
