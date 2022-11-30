mod config;

pub mod prelude {
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    pub use crate::config::Configuration;
}

use crate::prelude::*;
use bytes::{Buf, BufMut, BytesMut};
use std::io::{stdin, ErrorKind};
use std::str::from_utf8;
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::time::{sleep, Duration};
use tracing::{error, info};

pub async fn udp_server() -> Result<()> {
    let config = Configuration::from_file("./Config.toml");
    let socket = UdpSocket::bind(format!("{}:{}", config.ip, config.port)).await?;
    info!("Starting the server on {:?}", socket.local_addr()?);
    let mut buffer = BytesMut::with_capacity(8192);

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(1000)).await;
            info! {"alarm!"};
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

pub async fn udp_client() -> Result<()> {
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

pub async fn tcp_server() -> Result<()> {
    let config = Configuration::from_file("./Config.toml");
    let listener = TcpListener::bind(format!("{}:{}", config.ip, config.port)).await?;

    loop {
        let (stream, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = BytesMut::with_capacity(1024);
            loop {
                match stream.try_read_buf(&mut buffer) {
                    Ok(0) => {
                        error!("Readhalf of the stream is closed.");
                        break;
                    }
                    Ok(n) => {
                        info!("From: {}", stream.local_addr().unwrap());
                        let len = buffer.get_u64() as usize;
                        let text = from_utf8(&buffer[0..len]).unwrap();
                        info!("Received {} bytes: {:?}", n, text);
                        buffer.advance(len);
                        info!("===================================");
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
        });
    }
}

pub async fn tcp_client() -> Result<()> {
    let config = Configuration::from_file("./Config.toml");
    let stream = TcpStream::connect(format!("{}:{}", config.ip, config.port)).await?;
    let mut buffer = BytesMut::with_capacity(1024);

    loop {
        let mut text = String::new();
        stdin().read_line(&mut text).unwrap();

        buffer.put_u64(text.trim().len() as u64);
        buffer.put_slice(text.trim().as_bytes());
        stream.try_write(buffer.as_ref())?;
        buffer.clear();
    }
}
