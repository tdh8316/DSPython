use std::env;
use std::io::Write;
use std::process::Command;

/// Upload hex file to serial port
pub fn upload_to(hex: &str, port: &str) {
    print!("Uploading {} >> {}...", hex, port);
    std::io::stdout().flush().unwrap_or_default();

    // Load the environmental variable: `ARDUINO_DIR`
    let _arduino_dir = env::var("ARDUINO_DIR").expect(
        "You must set the environment variable 'ARDUINO_DIR' as your arduino software location",
    );
    let arduino_dir = _arduino_dir.as_str();

    // DSPython executable path
    let exe = env::current_exe().unwrap();
    let dir = exe.parent().expect("Executable must be in some directory");

    // Uploader path
    let uploader = if cfg!(debug_assertions) {
        format!("python src/uploader.py")
    } else {
        format!("{}/bin/uploader", dir.display())
    };
    let uploader = uploader.as_str();

    // Execute the uploader command
    let out = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", uploader, arduino_dir, hex, port])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("sh")
            .args(&["-c", uploader, arduino_dir, hex, port])
            .output()
            .expect("Failed to execute command")
    };
    if !out.status.success() {
        println!("{}", String::from_utf8_lossy(&out.stderr));
        panic!("Failed to perform uploading.");
    }
    println!("[Done]")
}
