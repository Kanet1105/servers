use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::sync::Mutex;

pub struct AppData {
    // Default hasher for the application.
    hasher: Mutex<Sha256>,
    // Offset for UTC.
    fixed_offset: FixedOffset,
    join_key: Mutex<String>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            hasher: Mutex::new(Sha256::new()),
            fixed_offset: FixedOffset::east(9 * 3600),
            join_key: Mutex::new(String::from("1234")),
        }
    }

    pub fn datetime(&self) -> DateTime<FixedOffset> {
        Utc::now().with_timezone(&self.fixed_offset)
    }

    pub fn datetime_vec(&self) -> Vec<u8> {
        Vec::<u8>::new()
    }

    pub fn join_key(&self) -> String {
        self.join_key.lock().unwrap().to_string()
    }
}
