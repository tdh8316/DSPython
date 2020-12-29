use std::env;
use std::process::{Command, Stdio};

pub fn get_arduino_dir() -> String {
    let _arduino_dir = env::var("ARDUINO_DIR").expect(
        "You must set the environment variable 'ARDUINO_DIR' as your arduino software location!",
    );
    let arduino_dir = _arduino_dir.to_string();

    return arduino_dir;
}

/// Generate non-linked object file from llvm ir
pub fn static_compiler(ir_path: &str, processor: &str, optimization_level: u8) -> String {
    let out = format!("{}.o", ir_path);
    let output = format!("-o={}", &out);
    let processor = format!("-mcpu={}", processor);
    let optimization_level = format!("-O{}", optimization_level);
    let mut args = vec![
        "llc",
        "-filetype=obj",
        ir_path,
        &output,
        &optimization_level,
        "--march=avr",
        &processor,
        "--thread-model=single",
    ];
    let mut process = if cfg!(target_os = "windows") {
        args.insert(0, "/C");
        println!("{:?}", args.join(" "));
        Command::new("cmd")
            .args(args.as_slice())
            .stdout(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Failed to execute llc!")
    } else {
        args.insert(0, "-c");
        println!("{:?}", args.join(" "));
        Command::new("sh")
            .args(args.as_slice())
            .stdout(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Failed to execute llc!")
    };
    let status = process.wait().unwrap();
    if !status.success() {
        eprintln!("ERROR: llc failed");
    }

    return out;
}
