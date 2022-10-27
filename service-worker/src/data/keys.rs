use chrono::prelude::*;
use sha2::{Sha256, Digest};
use std::{
    collections::HashMap,
};
use tracing::info;

pub struct KeyGen {
    hasher: Sha256,
}

impl KeyGen {
    pub fn join_key(&mut self) {
        
    }
}

impl Default for KeyGen {
    fn default() -> Self {
        Self {
            hasher: Sha256::new(),
        }
    }
}
