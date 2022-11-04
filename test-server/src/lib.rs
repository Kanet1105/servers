mod config;
mod data;
mod message;

use config::Configuration;
use data::AppData;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use tokio_stream::StreamExt;
use tracing::info;

pub type Exception = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Exception>;

pub async fn run() -> Result<()> {
    let data = AppData::new().with_config("./Config.toml").init();
    let ip_port = format!("{}:{}", data.config.ip, data.config.port);

    // Bind
    let listener = TcpListener::bind(&ip_port).await?;
    info!("The server's running on '{}'", ip_port);

    loop {
        let (stream, address) = listener.accept().await?;
        info!("Connected from {:?}", &address);
        
        let (mut reader, mut writer) = io::split(stream);
        let mut buffer = vec![0; 16];

        tokio::spawn(async move {
            loop {
                let n = match reader.read(&mut buffer).await {
                    Ok(n) => n,
                    Err(e) => {
                        info!("{:?}", e);
                        break;
                    },
                };
                if n == 0 {
                    break;
                }
                info!("Got {:?}", &buffer[..n]);
            }
        });
    }

    Ok(())
}

pub async fn connection_handler(stream: TcpStream) {
    let (mut stream_reader, stream_writer) = io::split(stream);
    let mut buffer = vec![0; 16];

    tokio::spawn(async move {
        loop {
            let n = match stream_reader.read(&mut buffer).await {
                Ok(n) => n,
                Err(e) => {
                    info!("{:?}", e);
                    break;
                },
            };
            if n == 0 {
                break;
            }
            info!("Message {:?}", &buffer[..n]);
        }
    });
}

pub struct Connection {
    id: String,
    stream :TcpStream,
    buffer: Vec<u8>,
}

impl Connection {

}
