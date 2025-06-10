use actix_web::{web, Responder};
use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};

static TOKENS_DISTRIBUTED: AtomicU64 = AtomicU64::new(0);
static TX_COUNT: AtomicU64 = AtomicU64::new(0);

pub async fn get_network_stats() -> impl Responder {
    web::Json(json!({
        "active_nodes": 0,  // TODO: Implement node tracking
        "tx_count": TX_COUNT.load(Ordering::Relaxed),
        "test_tokens_distributed": TOKENS_DISTRIBUTED.load(Ordering::Relaxed)
    }))
}

pub fn increment_tokens_distributed(amount: u64) {
    TOKENS_DISTRIBUTED.fetch_add(amount, Ordering::Relaxed);
}

pub fn increment_tx_count() {
    TX_COUNT.fetch_add(1, Ordering::Relaxed);
}