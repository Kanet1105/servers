use tracing_subscriber::fmt::time;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_timer(time::UtcTime::rfc_3339())
        .init();
    template::run_app().await?;
    Ok(())
}
