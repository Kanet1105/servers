mod configs;
mod data;

use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Responder};
use configs::Configs;
use data::AppData;
use tracing::info;

pub async fn health_check(request: HttpRequest) -> impl Responder {
    info!("{:?}", request);
    HttpResponse::Ok()
}

pub async fn join_key(request: HttpRequest, app_data: web::Data<AppData>) -> impl Responder {
    // Add a keygen
    HttpResponse::Ok()
}

pub async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let configs = Configs::from_file("Configs.toml");
    info!("Running a service server at '{}:{}'", configs.ip, configs.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppData::new()))
            .route("/health_check", web::get().to(health_check))
            .route("/join_key", web::get().to(join_key))
    })
    .bind((configs.ip, configs.port))?
    .run()
    .await?;
    Ok(())
}
