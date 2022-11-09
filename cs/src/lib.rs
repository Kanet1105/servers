mod config;

pub mod prelude {
    pub use crate::config::Configuration;

    pub use bytes::{Buf, BufMut, BytesMut};
    pub use std::{
        io::{stdin, ErrorKind},
        net::SocketAddr,
        str::from_utf8,
    };
    pub use tokio::{
        net::UdpSocket,
        sync::mpsc,
        task::spawn_blocking,
        time::{sleep, Duration, Instant},
    };
    pub use tracing::{error, info};

    pub type Error = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Error>;
}

use crate::prelude::*;

pub async fn run_server() -> Result<()> {
    let config = Configuration::from_file("./Config.toml");
    let socket = UdpSocket::bind(format!("{}:{}", config.ip, config.port)).await?;
    info!("Starting the server on {:?}", socket.local_addr()?);
    let mut buffer = BytesMut::with_capacity(8192);

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(1000)).await;
            info!{"alarm!"};
        }
    });

    loop {
        match socket.try_recv_buf_from(&mut buffer) {
            Ok((n, addr)) => {
                let mut data: BytesMut = buffer[0..n].into();
                tokio::spawn(async move {
                    info!("From: {}", addr);
                    let len = data.get_u64() as usize;
                    info!("Message length: {}", len);
                    let text = from_utf8(&data[0..len]).unwrap();
                    info!("Received {} bytes: {:?}", n, text);
                    data.advance(len);
                    info!("===================================");
                });
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                error!("{}", e);
                break;
            }
        }
    }
    Ok(())
}

pub async fn run_client() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let address = format!("127.0.0.1:{}", args[1]);

    let socket = UdpSocket::bind(&address).await?;
    info!("Starting the client on {}", address);
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
