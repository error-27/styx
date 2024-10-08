use std::{
    io::{self, Write},
    process::Command,
};

#[derive(Clone, Copy)]
pub struct LaunchOptions {
    pub pistol: bool,
    pub fast: bool,
    pub respawn: bool,
    pub turbo: usize,
}

impl Default for LaunchOptions {
    fn default() -> Self {
        Self {
            pistol: false,
            fast: false,
            respawn: false,
            turbo: 0,
        }
    }
}

pub fn launch_port(
    port: String,
    iwad: String,
    pwads: Vec<String>,
    complevel: isize,
    opts: LaunchOptions,
) {
    let mut command = Command::new(port);

    command.arg("-iwad").arg(iwad);

    if pwads.len() > 0 {
        command.arg("-file");
        for p in pwads {
            command.arg(p);
        }
    }

    if complevel != -1 {
        command.arg("-complevel").arg(complevel.to_string());
    }

    if opts.pistol {
        command.arg("-pistolstart");
    }
    if opts.fast {
        command.arg("-fast");
    }
    if opts.respawn {
        command.arg("-respawn");
    }
    if opts.turbo != 0 {
        command.arg("-turbo").arg(opts.turbo.to_string());
    }

    let out = command.output().expect("failed to run doom");
    io::stdout().write_all(&out.stdout).unwrap();
    io::stderr().write_all(&out.stderr).unwrap();
}
