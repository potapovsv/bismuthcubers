use actix_web::{web, App, HttpServer};
use tracing::info;
use tracing_subscriber::fmt;

mod xmla;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fmt::init();

    info!("Starting BismuthCubeRS server on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/xmla").route(web::post().to(xmla::handler::xmla_handler)))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}