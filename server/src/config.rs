use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub root_dir: String,
    pub secret: Option<String>,
}

impl Config {
    pub fn new(path: &str) -> Self {
        Config::from_config_file(path).unwrap()
    }
}
