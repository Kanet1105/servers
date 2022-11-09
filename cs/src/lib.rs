mod config;
pub mod prelude {
    pub type Error = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Error>;
}

use crate::prelude::*;
use config::Configuration;
use tokio::net::UdpSocket;
use tracing::info;

pub async fn run_udp() -> Result<()> {
    let config = Configuration::from_file("./Config.toml");
    let sock = UdpSocket::bind(format!("{}:{}", config.ip, config.port)).await?;
    info!("Starting the server on {:?}", sock.local_addr()?);

    loop {}
    Ok(())
}
