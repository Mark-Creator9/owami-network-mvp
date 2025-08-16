use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

/// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Owami Network API v0.1")
}

use owami_network_simplified::api_simplified as api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Simplified Owami Network - Entry Point Reached");
    
    println!("Creating HttpServer...");
    let _server = HttpServer::new(move || {
        println!("Creating App instance...");
        let batch_processor = web::Data::new(api::BatchProcessor::new());
        App::new()
            .app_data(batch_processor.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec!["content-type", "authorization", "accept"])
                    .expose_headers(vec!["content-disposition"])
            )
            .route("/", web::get().to(health_check))
            .service(
                web::scope("/api")
                    .configure(|cfg| api::config(cfg, batch_processor.clone()))
            )
            .service(Files::new("/", "landing").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8081))?
    .workers(4)
    .run()
    .await;
    Ok(())
}