use std::fs::create_dir_all;
use std::process::{Command, Stdio};

use crate::get_arduino_dir;

#[derive(Clone)]
pub struct AVRCompilerFlags {
    pub cpu_f: u64,
    pub mcu: String,
}

impl AVRCompilerFlags {
    pub fn new(cpu_f: u64, mcu: String) -> Self {
        AVRCompilerFlags { cpu_f, mcu }
    }
}

/// Generate full-linked hex file from an object and return the file path
pub fn avrgcc(object: &str, flags: AVRCompilerFlags) -> String {
    // Load the environmental variable: `ARDUINO_DIR`
    let arduino_dir = get_arduino_dir();

    // avr-gcc from Arduino IDE
    let gcc_executable = format!("{}/{}", arduino_dir, "/hardware/tools/avr/bin/avr-gcc");
    // avr-g++ from Arduino IDE
    let gpp_executable = format!("{}/{}", arduino_dir, "/hardware/tools/avr/bin/avr-g++");
    // avr-ar from Arduino IDE
    let ar_executable = format!("{}/{}", arduino_dir, "/hardware/tools/avr/bin/avr-ar");
    // avr-objcopy from Arduino IDE
    let objcopy_executable = format!("{}/{}", arduino_dir, "/hardware/tools/avr/bin/avr-objcopy");

    let mcu = format!("-mmcu={}", flags.mcu);
    let cpu_f = format!("-DF_CPU={}L", flags.cpu_f);
    let gcc_flags = vec![
        "-c",
        "-g",
        "-Os",
        "-Wall",
        "-ffunction-sections",
        "-fdata-sections",
        &mcu,
        &cpu_f,
        "-MMD",
        "-DUSB_VID=null",
        "-DUSB_PID=null",
        "-DARDUINO=106",
    ];
    let mut gpp_flags = gcc_flags.clone();
    gpp_flags.push("-fno-exceptions");

    // Headers
    let include_files = vec![
        format!("-I{}/hardware/arduino/avr/cores/arduino", arduino_dir),
        format!("-I{}/hardware/arduino/avr/variants/standard", arduino_dir),
        format!("-I{}/hardware/tools/avr/avr/include", arduino_dir),
    ];

    // Arduino headers
    let arduino_libs = format!("{}/hardware/arduino/avr/cores/arduino/", arduino_dir);

    let out_prefix = "arduino_build/";

    // Create a directory contains build files
    create_dir_all(out_prefix).unwrap();

    let compile_commands = vec![
        // Compile standard Arduino cores
        "{GCC} {GCC_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}WInterrupts.c -o {OUT_PREFIX}WInterrupts.c.o",
        "{GCC} {GCC_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}wiring.c -o {OUT_PREFIX}wiring.c.o",
        "{GCC} {GCC_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}wiring_analog.c -o {OUT_PREFIX}wiring_analog.c.o",
        "{GCC} {GCC_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}wiring_digital.c -o {OUT_PREFIX}wiring_digital.c.o",
        "{GCC} {GCC_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}wiring_pulse.c -o {OUT_PREFIX}wiring_pulse.c.o",
        "{GCC} {GCC_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}wiring_pulse.S -o {OUT_PREFIX}wiring_pulse.S.o",
        "{GCC} {GCC_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}wiring_shift.c -o {OUT_PREFIX}wiring_shift.c.o",
        "{GCC} {GCC_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}hooks.c -o {OUT_PREFIX}hooks.c.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}CDC.cpp -o {OUT_PREFIX}CDC.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}HardwareSerial.cpp -o {OUT_PREFIX}HardwareSerial.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}IPAddress.cpp -o {OUT_PREFIX}IPAddress.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}main.cpp -o {OUT_PREFIX}main.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}new.cpp -o {OUT_PREFIX}new.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}Print.cpp -o {OUT_PREFIX}Print.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}Stream.cpp -o {OUT_PREFIX}Stream.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}Tone.cpp -o {OUT_PREFIX}Tone.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}USBCore.cpp -o {OUT_PREFIX}USBCore.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}WMath.cpp -o {OUT_PREFIX}WMath.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}WString.cpp -o {OUT_PREFIX}WString.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}PluggableUSB.cpp -o {OUT_PREFIX}PluggableUSB.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}HardwareSerial0.cpp -o {OUT_PREFIX}HardwareSerial0.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}HardwareSerial1.cpp -o {OUT_PREFIX}HardwareSerial1.cpp.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} {ARDUINO_LIBS}abi.cpp -o {OUT_PREFIX}abi.cpp.o",

        // Compile DSPython wrapper
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} include/Serial.cc -o {OUT_PREFIX}Serial.cc.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} include/Builtins.cc -o {OUT_PREFIX}Builtins.cc.o",
        "{GPP} {GPP_FLAGS} {INCLUDE_FILES} include/LLVMArduinoBuiltins.cc -o {OUT_PREFIX}LLVMArduinoBuiltins.cc.o",

        // Archiver
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}WInterrupts.c.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}wiring.c.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}wiring_analog.c.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}wiring_digital.c.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}wiring_pulse.c.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}wiring_pulse.S.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}wiring_shift.c.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}hooks.c.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}CDC.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}HardwareSerial.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}IPAddress.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}main.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}new.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}Print.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}Stream.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}Tone.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}USBCore.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}WMath.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}WString.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}PluggableUSB.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}HardwareSerial0.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}HardwareSerial1.cpp.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}abi.cpp.o",

        // Link DSPython wrapper library
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}Serial.cc.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}Builtins.cc.o",
        "{AR} rcs {OUT_PREFIX}core.a {OUT_PREFIX}LLVMArduinoBuiltins.cc.o",

        // Compile everything together
        "{GCC} -w -Os -g -flto -fuse-linker-plugin -Wl,--gc-sections -mmcu={MCU} -o {INPUT}.elf {INPUT} {OUT_PREFIX}core.a -lm",
        "{OBJCOPY} -O ihex -j .eeprom --set-section-flags=.eeprom=alloc,load --no-change-warnings --change-section-lma .eeprom=0 {INPUT}.elf {INPUT}.eep",
        "{OBJCOPY} -O ihex -R .eeprom {INPUT}.elf {INPUT}.hex",
    ];

    for command in compile_commands {
        let command_string = command
            .replace("{GCC}", &gcc_executable)
            .replace("{GCC_FLAGS}", &gcc_flags.join(" "))
            .replace("{INCLUDE_FILES}", &include_files.join(" "))
            .replace("{ARDUINO_LIBS}", &arduino_libs)
            .replace("{OUT_PREFIX}", out_prefix)
            .replace("{GPP}", &gpp_executable)
            .replace("{GPP_FLAGS}", &gpp_flags.join(" "))
            .replace("{AR}", &ar_executable)
            .replace("{MCU}", &flags.mcu)
            .replace("{OBJCOPY}", &objcopy_executable)
            .replace("{INPUT}", object);
        let mut args = command_string.as_str().split(" ").collect::<Vec<&str>>();

        let mut process = if cfg!(target_os = "windows") {
            args.insert(0, "/C");
            Command::new("cmd")
                .args(args.as_slice())
                .stdout(Stdio::inherit())
                .stdout(Stdio::inherit())
                .spawn()
                .expect(&format!("Failed to execute command '{}'", command_string.as_str()))
        } else {
            args.insert(0, "-c");
            Command::new("sh")
                .args(args.as_slice())
                .stdout(Stdio::inherit())
                .stdout(Stdio::inherit())
                .spawn()
                .expect(&format!("Failed to execute command '{}'", command_string.as_str()))
        };
        let status = process.wait().unwrap();
        if !status.success() {
            eprintln!("'{}' returned non-zero status {}", command_string.as_str(), status.code().unwrap_or(-1));
            panic!("ERROR: avrgcc failed");
        }
    }

    return format!("{}.hex", object);
}
