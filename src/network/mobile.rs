use crate::blockchain::Blockchain;
use crate::consensus::dpos::{self, LightClientRequest, LightClientResponse};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct MobileSyncRequest {
    pub last_known_height: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MobileSyncResponse {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionProofRequest {
    pub transaction_hash: String,
    pub block_height: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionProofResponse {
    pub proof: Vec<String>,
    pub verified: bool,
}

pub async fn mobile_sync(
    State(_blockchain): State<Arc<Mutex<Blockchain>>>,
    Json(_request): Json<MobileSyncRequest>,
) -> Result<Json<MobileSyncResponse>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn verify_transaction(
    State(_blockchain): State<Arc<Mutex<Blockchain>>>,
    Json(_request): Json<TransactionProofRequest>,
) -> Result<Json<TransactionProofResponse>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn light_client_handler(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
    Json(request): Json<LightClientRequest>,
) -> Json<LightClientResponse> {
    let blockchain = blockchain.lock().await;
    let response = dpos::handle_light_request(request, &blockchain.consensus);
    Json(response)
}
