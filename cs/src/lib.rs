mod config;

pub mod prelude {
    pub type Error = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Error>;

    pub use crate::config::Configuration;

    pub use bytes::{Buf, BufMut, BytesMut};
    pub use std::{
        io::{stdin, ErrorKind},
        str::from_utf8,
    };
    pub use tokio::net::UdpSocket;
    pub use tracing::{error, info};
}

use crate::prelude::*;

pub async fn run_server() -> Result<()> {
    let config = Configuration::from_file("./Config.toml");
    let socket = UdpSocket::bind(format!("{}:{}", config.ip, config.port)).await?;
    info!("Starting the server on {:?}", socket.local_addr()?);
    let mut buffer = BytesMut::with_capacity(8192);

    loop {
        match socket.try_recv_buf(&mut buffer) {
            Ok(n) => {
                let len = buffer.get_u64() as usize;
                info!("Message length : {}", len);
                let text = from_utf8(&buffer[0..len])?;
                info!("Received {} bytes: {:?}", n, text);
                buffer.advance(len);
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            },
            Err(e) => {
                error!("{}", e);
                return Err(Box::new(e));
            },
        }
    }
}

pub async fn run_client() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:50001").await?;
    socket.connect("127.0.0.1:50000").await?;
    let mut buffer = BytesMut::with_capacity(8);

    loop {
        let mut text = String::new();
        stdin().read_line(&mut text).unwrap();
        
        buffer.put_u64(text.trim().len() as u64);
        buffer.put_slice(text.trim().as_bytes());
        socket.try_send(buffer.as_ref())?;
        buffer.clear();
    }
}
