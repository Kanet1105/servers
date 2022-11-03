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
};
use tracing::info;

pub type Exception = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Exception>;

pub async fn run() -> Result<()> {
    let data = AppData::new().with_config("./Config.toml").init();
    let ip_port = format!("{}:{}", data.config.ip, data.config.port);
    let listener = TcpListener::bind(&ip_port).await?;
    info!("The server's running on '{}'", ip_port);

    loop {
        let (mut stream, address) = listener.accept().await?;
        stream.shutdown().await?;
        
        // let (mut reader, mut writer) = io::split(stream);
        // let mut buffer = vec![0; 16];
        // info!("Connected from {:?}", &address);

        // tokio::spawn(async move {
        //     loop {
        //         let n = match reader.read(&mut buffer).await {
        //             Ok(n) => n,
        //             Err(e) => {
        //                 info!("{:?}", e);
        //                 break;
        //             },
        //         };
        //         if n == 0 {
        //             break;
        //         }
        //         info!("Got {:?}", &buffer[..n]);
        //     }
        // });
    }

    Ok(())
}

pub async fn handler() -> Result<()> {
    Ok(())
}
