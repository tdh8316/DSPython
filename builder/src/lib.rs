use std::env;
use std::io::Write;
use std::process::Command;

/// Generate hex file from llvm assembly and return the file path
pub fn objcopy(assembly: &str) -> String {
    print!("Generating hex file...");
    std::io::stdout().flush().unwrap_or_default();

    // Load the environmental variable: `ARDUINO_DIR`
    let _arduino_dir = env::var("ARDUINO_DIR").expect(
        "You must set the environment variable 'ARDUINO_DIR' as your arduino software location",
    );
    let arduino_dir = _arduino_dir.as_str();

    // DSPython executable path
    let exe = env::current_exe().unwrap();
    let dir = exe.parent().expect("Executable must be in some directory");

    // Linker path
    let linker = if cfg!(debug_assertions) {
        format!("python builder/linker.py")
    } else {
        format!("{}/bin/builder", dir.display())
    };
    let linker = linker.as_str();

    // Execute the linker command
    let out = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", linker, arduino_dir, assembly])
            .status()
            .expect("Failed to execute command")
    } else {
        Command::new("sh")
            .args(&["-c", linker, arduino_dir, assembly])
            .status()
            .expect("Failed to execute command")
    };
    if !out.success() {
        panic!("Failed to perform builder.");
    }
    println!("[Done]");
    {
        format!("{}.hex", assembly)
    }
}
