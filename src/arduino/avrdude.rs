use std::process::{Command, Stdio};

use crate::utils::get_arduino_dir;

#[derive(Clone)]
pub struct AVRDudeFlags {
    pub mcu: String,
    pub port: String,
    pub baudrate: u64,
}

impl AVRDudeFlags {
    pub fn new(mcu: String, port: &str, baudrate: u64) -> Self {
        AVRDudeFlags {
            mcu,
            port: port.to_string(),
            baudrate,
        }
    }
}

/// Use avrdude to flash hex
pub fn avrdude(target_path: &str, flags: AVRDudeFlags) {
    // Load the environmental variable: `ARDUINO_DIR`
    let arduino_dir = get_arduino_dir();

    // avrdude from Arduino IDE
    let avrdude_executable = format!("{}/{}", arduino_dir, "hardware/tools/avr/bin/avrdude");

    let config_file = format!("{}/{}", arduino_dir, "/hardware/tools/avr/etc/avrdude.conf");
    let mcu = format!("-p{}", flags.mcu);
    let port = format!("-P{}", flags.port);
    let b = format!("-b{}", flags.baudrate);
    let memtype = format!("-Uflash:w:{}:i", target_path);

    let mut args = vec![
        // avrdude executable path
        avrdude_executable.as_str(),
        // Arduino configuration file for the avrdude
        &config_file,
        // verbose output
        "-v",
        // micro controller unit
        &mcu,
        // programmer type
        "-carduino",
        // serial port
        &port,
        // baudrate
        &b,
        // Disable auto erase for flash memory
        "-D",
        // memory operation specification
        &memtype,
    ];

    let mut process = if cfg!(target_os = "windows") {
        println!("{}", args.join(" "));
        args.insert(0, "/C");
        Command::new("cmd")
            .args(args.as_slice())
            .stdout(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Failed to execute avrdude!")
    } else {
        println!("{}", args.join(" "));
        args.insert(0, "-c");
        Command::new("sh")
            .args(args.as_slice())
            .stdout(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Failed to execute avrdude!")
    };
    let status = process.wait().unwrap();
    if !status.success() {
        panic!("ERROR: avrdude returned non-zero status {}", status.code().unwrap_or(-1));
    }
}
