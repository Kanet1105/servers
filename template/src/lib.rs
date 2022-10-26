mod configs;

use actix_web::{get, post, App, HttpServer};
use configs::Configs;
use tracing::info;

pub async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let configs = Configs::from_file("Configs.toml");
    info!("Running a service server at '{}:{}'", configs.ip, configs.port);
    HttpServer::new(move || {
        App::new()
    })
    .bind((configs.ip, configs.port))?
    .run()
    .await?;
    Ok(())
}
