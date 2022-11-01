mod config;

use bytes::Bytes;
use config::Configuration;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, AsyncReadExt},
    net::TcpListener,
};
use tracing::info;

pub type Exception = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Exception>;

pub async fn run() -> Result<()> {
    let config = Arc::new(Configuration::from_file("./Config.toml"));
    let listener = TcpListener::bind(&format!("{}:{}", config.ip, config.port)).await?;
    info!("The server's running on '{}:{}'", config.ip, config.port);

    loop {
        let (stream, address) = listener.accept().await?;
        let (mut reader, mut writer) = io::split(stream);
        let mut buffer = vec![0; 128];
        info!("Connected from {:?}", &address);

        tokio::spawn(async move {
            loop {
                let n = reader.read(&mut buffer).await.unwrap();
                if n == 0 {
                    break;
                }
                info!("Got {:?}", &buffer[..n]);
            }
        });
    }

    Ok(())
}
