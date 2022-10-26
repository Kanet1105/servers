use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf,};
use tracing::info;

#[derive(Deserialize, Serialize)]
pub struct Configs {
    // IP and the port where the service server binds to.
    pub ip: String,
    pub port: u16,
}

impl Default for Configs {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

impl Configs {
    pub fn from_file(name: &str) -> Self {
        let path = PathBuf::from(name);
        if path.exists() {
            let file = fs::read_to_string(path).expect("Failed to read the config file.");
            let config: Configs = toml::from_str(&file).expect("Failed to parse the config file.");
            config
        } else {
            info!("Could not find 'Configs.toml' in the root dir. The server uses the default configs.");
            Self::default()
        }
    }
}
