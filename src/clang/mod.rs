use std::env;
use std::io::ErrorKind::NotFound;
use std::process::Command;

pub struct Clang {
    executable: String,
}

impl Clang {
    pub fn new() -> Self {
        let executable = if let NotFound = Command::new("clang").spawn().err().unwrap().kind() {
            if let NotFound = Command::new("./bin/clang").spawn().err().unwrap().kind() {
                let pref =
                    env::var("LLVM_SYS_130_PREFIX").expect("Failed to get an executable clang");
                format!("{}/bin/clang", pref)
            } else {
                "./bin/clang".to_string()
            }
        } else {
            "clang".to_string()
        };
        Self { executable }
    }

    pub fn run(&self, args: &[&str]) -> Result<(), String> {
        let mut command = Command::new(&self.executable);
        command.args(args);
        let output = command.output().unwrap();
        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}
