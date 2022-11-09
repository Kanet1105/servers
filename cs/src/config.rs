use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub ip: String,
    pub port: u16,
    pub concurrency_limit: u16,
}

impl Configuration {
    pub fn from_file(name: &str) -> Self {
        let path = PathBuf::from(name);

        if path.exists() {
            let file = fs::read_to_string(path).expect("Failed to read the config file.");
            let config: Configuration =
                toml::from_str(&file).expect("Failed to parse the config file.");
            config
        } else {
            Self::default()
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: 50000,
            concurrency_limit: 10000,
        }
    }
}
