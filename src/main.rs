use actix_cors::Cors;
use std::sync::Mutex;
use owami_network::vesting::VestingManager;
use owami_network::db::create_pool;
use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use owami_network::auth::create_jwt_config;

/// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Owami Network API v0.1")
}

#[cfg(feature = "production")]
fn setup_logging() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Owami Network - Entry Point Reached");
    #[cfg(feature = "production")]
    setup_logging();
    println!("After logging setup");
    
    let jwt_config = create_jwt_config();

    // Initialize VestingManager
    let vesting_manager = std::sync::Arc::new(Mutex::new(VestingManager::default()));

    // Initialize Postgres pool
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:owamitest@localhost:5432/owami-network".to_string());
    let db_pool = create_pool(&database_url).await.expect("Failed to create database pool");

    log::info!("Starting Owami Network server");

    println!("Creating HttpServer...");
    let _server = HttpServer::new(move || {
        println!("Creating App instance...");
        let batch_processor = web::Data::new(owami_network::api::BatchProcessor::new());
        let db_pool_data = web::Data::new(db_pool.clone());
        App::new()
            .app_data(db_pool_data)
            .service(
                Files::new("/", "./landing")
                    .index_file("index.html")
                    .prefer_utf8(true)
                    .show_files_listing()
            )
            .service(
                web::resource("/").to(|| async {
                    actix_web::HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("../landing/index.html"))
                })
            )
            .app_data(web::Data::new(jwt_config.clone()))
            .app_data(web::Data::from(vesting_manager.clone()))
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
                    .configure(|cfg| owami_network::api::config(cfg, batch_processor.clone()))
            )
            .service(Files::new("/", "./")
                .index_file("landing/index.html")
                .prefer_utf8(true))
    })
    .bind(("0.0.0.0", 8081))?
    .workers(4)
    .run()
    .await;
    Ok(())
}
