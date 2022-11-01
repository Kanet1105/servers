use std::io::stdin;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::info;

pub type Exception = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Exception>;

#[tokio::main]
async fn main() -> Result<()> {
    let stream = TcpStream::connect("127.0.0.1:50000").await?;
    let (mut reader, mut writer) = io::split(stream);
    let mut buffer = vec![0; 128];

    tokio::spawn(async move {
        loop {
            let n = reader.read(&mut buffer).await.unwrap();
            if n == 0 {
                break;
            }
            info!("Got {:?}", &buffer[..n]);
        }
    });
    
    loop {
        // User input.
        let mut text = String::new();
        stdin().read_line(&mut text).unwrap();
        writer.write_all(&text.as_bytes()).await.unwrap();
    }
}
