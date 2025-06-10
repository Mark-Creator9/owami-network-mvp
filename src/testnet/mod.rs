pub mod monitoring;
pub mod faucet;
pub mod sandbox;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/testnet")
            .route("/stats", web::get().to(monitoring::get_network_stats))
            .configure(faucet::config)
            .configure(sandbox::config)
    );
}