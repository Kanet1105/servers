use crate::config::Configuration;
use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, Mutex},
};

pub struct SharedData(Arc<AppData>);

impl Clone for SharedData {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Deref for SharedData {
    type Target = Arc<AppData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct AppData {
    pub config: Configuration,
    pub concurrency: Mutex<u16>,
    pub channel: Mutex<HashMap<String, String>>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            config: Configuration::default(),
            concurrency: Mutex::new(0),
            channel: Mutex::new(HashMap::new()),
        }
    }

    pub fn with_config(mut self, config_path: &str) -> Self {
        self.config = Configuration::from_file(config_path);
        self
    }

    pub fn init(self) -> SharedData {
        SharedData(Arc::new(self))
    }
}
