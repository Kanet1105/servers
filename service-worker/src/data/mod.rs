mod keys;
pub use keys::*;

use std::{
    sync::{Arc, Mutex},
};

pub struct AppData {
    pub key_gen: KeyGen,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            key_gen: KeyGen::default(),
        }
    }
}
