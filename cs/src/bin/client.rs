use cs::prelude::*;
use tracing_subscriber::fmt::time;

#[tokio::main()]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_timer(time::LocalTime::rfc_3339())
        .init();
    cs::run_client().await?;
    Ok(())
}
