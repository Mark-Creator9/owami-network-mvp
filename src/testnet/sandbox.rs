use actix_web::{web, HttpResponse};

async fn sandbox_handler() -> HttpResponse {
    HttpResponse::Ok().body("Sandbox endpoint")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/sandbox")
            .route(web::post().to(sandbox_handler))
    );
}