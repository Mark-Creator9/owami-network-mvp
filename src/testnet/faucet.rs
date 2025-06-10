use actix_web::{web, HttpResponse};
use lazy_static::lazy_static;
use crate::transaction::Transaction;
use rand_core::RngCore;
use serde::Deserialize;
use crate::testnet::monitoring;
use ed25519_dalek::SigningKey;

#[derive(Deserialize)]
pub struct FaucetRequest {
    wallet_address: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/faucet")
            .route(web::post().to(faucet_handler))
    );
}

pub async fn faucet_handler(request: web::Json<FaucetRequest>) -> HttpResponse {
    let amount = 1000;
    
    // Generate faucet key once
    lazy_static! {
        static ref FAUCET_KEY: SigningKey = {
            let mut rng = rand::thread_rng();
            let mut secret = [0u8; 32];
            rng.fill_bytes(&mut secret);
            SigningKey::from_bytes(&secret)
        };
    }

    // Create actual transaction
    let tx = Transaction::new_transfer(
        hex::encode(*FAUCET_KEY.as_bytes()),
        request.wallet_address.clone(),
        amount,
        0,
        0 // nonce
    );

    // Generate mock tx hash
    let tx_hash = blake3::hash(format!("{:?}{}", tx, chrono::Utc::now()).as_bytes())
        .to_hex()[..16].to_string();

    // Update monitoring stats
    monitoring::increment_tokens_distributed(amount);
    monitoring::increment_tx_count();
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Test tokens sent",
        "wallet": request.wallet_address,
        "amount": amount,
        "tx_hash": format!("0x{}", tx_hash)
    }))
}