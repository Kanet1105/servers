mod config;
mod data;

use bytes::{BytesMut, Buf, BufMut};
use data::{AppData, SharedData};
use message::{deserialize_with_capacity, SimpleText};
use std::{
    net::SocketAddr,
};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt, BufReader},
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
    let listener = TcpListener::bind(&ip_port).await?;
    info!("The server's running on '{}'", ip_port);

    loop {
        let (stream, address) = listener.accept().await?;
        connection_handler(stream, address, data.clone());
    }
}

pub fn connection_handler(stream: TcpStream, address: SocketAddr, data: SharedData) {
    let (mut read_stream, mut write_stream) = io::split(stream);
    let mut buffer = BytesMut::with_capacity(8196);
    let mut data = SimpleText {
        channel: "".into(),
        contents: "".into(),
    };

    // Broadcast to the channel belong.
    tokio::spawn(async move {
        loop {
            let n = match read_stream.read_buf(&mut buffer).await {
                Ok(n) => n,
                Err(e) => {
                    info!("{:?}", e);
                    break;
                }
            };
            if n == 0 {
                break;
            }
            info!("Read {} bytes", n);
            deserialize_with_capacity(&mut buffer, &mut data).unwrap();
            info!("{:?}", &data);
        }
    });

    info!("Connected from {:?}", address);
}
