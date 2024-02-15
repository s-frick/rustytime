use std::{fs, io::Write};

use config::{Config, File};
use glob::{glob, Paths};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(unused)]
pub struct Rustytime {
    pub home: String,
}

impl Default for Rustytime {
    fn default() -> Self {
        let home = shellexpand::full("~/.config/rustytime")
            .unwrap()
            .to_string();
        Self { home }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[allow(unused)]
pub struct Settings {
    pub rustytime: Rustytime,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        // TODO: make this more robust, handle case where rusty's home not yet exists
        let home = shellexpand::full("~/.config/rustytime").unwrap();
        let home_glob = format!("{}/config/*", home);
        let settings = Config::builder()
            .add_source(find_config(home_glob, home.to_string()))
            .build()?;

        settings.clone().try_deserialize()
    }
}

fn create_config(home: &str, home_glob: &str) -> Paths {
    let path = format!("{}/config/config.toml", home);
    let prefix = std::path::Path::new(path.as_str()).parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .unwrap();

    let s = Settings::default();
    f.write_all(toml::to_string_pretty(&s).unwrap().as_bytes())
        .unwrap();
    glob(home_glob).unwrap()
}

fn find_config(
    home_glob: String,
    home: String,
) -> Vec<File<config::FileSourceFile, config::FileFormat>> {
    let files = glob(home_glob.as_str())
        .unwrap()
        .map(|path| File::from(path.unwrap()))
        .collect::<Vec<_>>();

    match files.as_slice() {
        [] => create_config(home.as_str(), home_glob.as_str())
            .map(|path| File::from(path.unwrap()))
            .collect::<Vec<_>>(),
        _ => files,
    }
}
