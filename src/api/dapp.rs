use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateDAppPayload {
    pub name: String,
    pub description: String,
    pub contract_address: String,
}

#[derive(Serialize)]
pub struct DAppResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub contract_address: String,
    pub creator_id: String,
    pub created_at: String,
}

pub async fn create_dapp(Json(payload): Json<CreateDAppPayload>) -> Json<DAppResponse> {
    let creator_id = "default_creator".to_string(); // In a real implementation, this would come from auth

    Json(DAppResponse {
        id: uuid::Uuid::new_v4().to_string(),
        name: payload.name,
        description: payload.description,
        contract_address: payload.contract_address,
        creator_id,
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}
