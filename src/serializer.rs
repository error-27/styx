use dirs::config_dir;
use std::{
    fs::File,
    io::{Read, Write},
};

use crate::AppSettings;

pub fn save_settings(settings: &AppSettings) {
    let serialized = toml::to_string(settings).unwrap();
    println!("{}", serialized);

    let mut file = File::create(config_dir().unwrap().join("styx_conf.toml")).unwrap();
    file.write_all(serialized.as_bytes())
        .expect("failed to write serialized data");
}

pub fn load_settings() -> Result<AppSettings, std::io::Error> {
    let file_result = File::open(config_dir().unwrap().join("styx_conf.toml"));

    let mut file;
    if let Ok(f) = file_result {
        file = f;
    } else {
        return Ok(AppSettings::default());
    }
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let settings = toml::from_str(&buf).unwrap();

    Ok(settings)
}
