use actix_web::{get, App, HttpServer, Responder};
use log::{info, error};

#[get("/health")]
async fn health() -> impl Responder {
    "OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let addr = "127.0.0.1:3030";
    
    info!("Starting standalone HTTP server on {}", addr);
    
    HttpServer::new(|| {
        App::new()
            .service(health)
    })
    .bind(addr)?
    .run()
    .await
}