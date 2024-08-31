use std::process::Command;

pub struct LaunchOptions {
    pistol: bool,
    fast: bool
}

pub fn launch_port(port: String, iwad: String) {
    let command = Command::new(port)
        .arg("-iwad")
        .arg(iwad)
        .output()
        .expect("failed to run doom");
}