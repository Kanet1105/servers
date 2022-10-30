mod config;
mod server;

use config::Configuration;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

pub type Exception = Box<dyn std::error::Error>;

pub async fn run() -> Result<(), Exception> {
    let config = Arc::new(Configuration::from_file("./Config.toml"));
    let listener = TcpListener::bind(&format!("{}:{}", config.ip, config.port)).await.unwrap();

    tokio::spawn(async {

    });
    Ok(())
}
