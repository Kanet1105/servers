use test_server::Result;
use tracing_subscriber::fmt::time;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_timer(time::LocalTime::rfc_3339())
        .init();
    test_server::run().await?;
    Ok(())
}
