use cs::prelude::*;
use tracing_subscriber::fmt::time;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_timer(time::LocalTime::rfc_3339())
        .init();
    cs::udp_server().await?;
    // cs::tcp_server().await?;
    Ok(())
}
