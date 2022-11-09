mod config;
pub mod prelude {
    pub type Error = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Error>;
}

use crate::prelude::*;

use bytes::BytesMut;
use config::Configuration;
use std::{
    io::ErrorKind,
    str::from_utf8,
};
use tokio::net::UdpSocket;
use tracing::{error, info};

pub async fn run_udp() -> Result<()> {
    let config = Configuration::from_file("./Config.toml");
    let sock = UdpSocket::bind(format!("{}:{}", config.ip, config.port)).await?;
    info!("Starting the server on {:?}", sock.local_addr()?);
    let mut buffer = BytesMut::with_capacity(8192);

    loop {
        match sock.try_recv_buf(&mut buffer) {
            Ok(n) => {
                let message = from_utf8(&buffer)?;
                info!("Received {} bytes: {}", n, message);
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
