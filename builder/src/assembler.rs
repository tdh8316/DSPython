use std::process::Command;

pub fn static_compiler(ir_path: &str) {
    let out =
        Command::new("cmd")
            .args(&[
                "/C", "llc", "-filetype=obj", ir_path, &("-o=".to_owned() + ir_path + ".o"),
                "-O3", "--march=avr", "-mcpu=atmega328p", "--thread-model=single"
            ])
            .status()
            .expect("Failed to execute command");
    if !out.success() {
        panic!("Failed to perform LLVM static compiler.");
    }
}
