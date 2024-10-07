use std::{
    io::{self, Write},
    process::Command,
};

pub struct LaunchOptions {
    pistol: bool,
    fast: bool,
}

pub fn launch_port(port: String, iwad: String, pwads: Vec<String>) {
    let mut command = Command::new(port);

    command.arg("-iwad").arg(iwad);

    if pwads.len() > 0 {
        command.arg("-file");
        for p in pwads {
            command.arg(p);
        }
    }

    let out = command.output().expect("failed to run doom");
    io::stdout().write_all(&out.stdout).unwrap();
    io::stderr().write_all(&out.stderr).unwrap();
}
