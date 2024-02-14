use config::{Config, File};
use glob::glob;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Rustytime {
    pub home: String,
}

#[derive(Debug, Deserialize, Clone)]
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
            .add_source(
                glob(home_glob.as_str())
                    .unwrap()
                    .map(|path| File::from(path.unwrap()))
                    .collect::<Vec<_>>(),
            )
            .build()?;

        settings.clone().try_deserialize()
    }
}
