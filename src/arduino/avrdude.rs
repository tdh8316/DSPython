use std::process::{Command, Stdio};

use crate::utils::get_arduino_dir;

#[derive(Clone)]
pub struct AVRDudeFlags {
    pub processor: String,
    pub port: String,
    pub baudrate: u64,
}

impl AVRDudeFlags {
    pub fn new(processor: String, port: &str, baudrate: u64) -> Self {
        AVRDudeFlags {
            processor,
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
    let processor = format!("-p{}", flags.processor);
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
        // AVR device
        &processor,
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
        args.insert(0, "/C");
        println!("{:?}", args.join(" "));
        Command::new("cmd")
            .args(args.as_slice())
            .stdout(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Failed to execute avrdude!")
    } else {
        args.insert(0, "-c");
        println!("{:?}", args.join(" "));
        Command::new("sh")
            .args(args.as_slice())
            .stdout(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Failed to execute avrdude!")
    };
    let status = process.wait().unwrap();
    if !status.success() {
        eprintln!("ERROR: avrdude failed");
    }
}
