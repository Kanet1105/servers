mod configs;
mod data;

use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Responder};
use actix_ws::Message;
use configs::Configs;
use data::AppData;
use futures::stream::StreamExt;
use tracing::info;

pub async fn health_check(request: HttpRequest) -> impl Responder {
    info!("{:?}", request);
    HttpResponse::Ok()
}

pub async fn join_key(request: HttpRequest, app_data: web::Data<AppData>) -> impl Responder {
    // info!("{:?}", app_data.join_key());
    HttpResponse::Ok()
}

pub async fn ws(request: HttpRequest, body: web::Payload) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let (response, mut session, mut message_stream) = actix_ws::handle(&request, body)?;
    
    actix_rt::spawn(async move {
        while let Some(Ok(message)) = message_stream.next().await {
            match message {
                Message::Text(s) => {
                    info!("{}", s);
                },
                _ => break,
            }
        }
        let _ = session.close(None).await;
    });
    Ok(response)
}

// pub async fn join(request: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Box<dyn std::error::Error>> {
//     let (response, session, message_stream) = actix_ws::handle(&request, stream)?;
// }

pub async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let configs = Configs::from_file("Configs.toml");
    info!("Running a service server at '{}:{}'", configs.ip, configs.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppData::new()))
            .route("/health_check", web::get().to(health_check))
            .route("/join", web::get().to(join_key))
    })
    .bind((configs.ip, configs.port))?
    .run()
    .await?;
    Ok(())
}
