use actix_multipart::Multipart;
use actix_web::{post, get, web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

// Import our modules
use crate::deploy::{DeploymentService, DeployRequest, DeployResponse, CallRequest, CallResponse};

/// Global deployment service
#[derive(Clone)]
pub struct AppState {
    pub deployment_service: Arc<Mutex<DeploymentService>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            deployment_service: Arc::new(Mutex::new(DeploymentService::new())),
        }
    }
}

/// Upload contract file endpoint
#[post("/upload")]
async fn upload_contract(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    let mut filepath = String::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        if let Some(filename) = content_disposition.get_filename() {
            let sanitized_filename = sanitize_filename::sanitize(filename);
            let filepath_str = format!("./uploads/{}", sanitized_filename);
            filepath = filepath_str.clone();
            
            // Create uploads directory if it doesn't exist
            std::fs::create_dir_all("./uploads").unwrap();
            
            let mut f = web::block(|| File::create(filepath_str)).await??;
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                f = web::block(move || {
                    let mut file = f;
                    file.write_all(&data)?;
                    Ok::<File, std::io::Error>(file)
                }).await??;
            }
        }
    }
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "filepath": filepath,
        "status": "uploaded"
    })))
}

/// Deploy contract endpoint
#[post("/deploy")]
async fn deploy_contract(
    data: web::Data<AppState>,
    req: web::Json<DeployRequest>,
) -> impl Responder {
    let deployment_service = data.deployment_service.lock().unwrap();
    
    // Decode base64 contract code
    let contract_code = match base64::decode(&req.contract_code) {
        Ok(code) => code,
        Err(e) => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": format!("Invalid base64: {}", e)}));
        }
    };

    match deployment_service
        .deploy_from_code(&req.contract_type, &contract_code, &req.network)
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": e})),
    }
}

/// Deploy contract from file endpoint
#[post("/deploy_file")]
async fn deploy_contract_file(
    data: web::Data<AppState>,
    req: web::Json<serde_json::Value>,
) -> impl Responder {
    let contract_path = match req.get("contract_path").and_then(|v| v.as_str()) {
        Some(path) => path,
        None => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": "Missing contract_path"}));
        }
    };

    let contract_type = match req.get("contract_type").and_then(|v| v.as_str()) {
        Some(t) => t,
        None => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"error": "Missing contract_type"}));
        }
    };

    let network = req.get("network").and_then(|v| v.as_str()).unwrap_or("testnet");

    let deployment_service = data.deployment_service.lock().unwrap();
    
    match deployment_service
        .deploy_from_file(contract_path, contract_type, network)
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": e})),
    }
}

/// Call contract endpoint
#[post("/call")]
async fn call_contract(
    data: web::Data<AppState>,
    req: web::Json<CallRequest>,
) -> impl Responder {
    let deployment_service = data.deployment_service.lock().unwrap();
    
    // Check if contract exists
    if deployment_service.get_contract(&req.contract_address).is_none() {
        return HttpResponse::NotFound()
            .json(serde_json::json!({"error": "Contract not found"}));
    }

    // For now, return a mock response
    HttpResponse::Ok().json(CallResponse {
        result: serde_json::json!({
            "message": format!("Called {} on contract {}", req.function_name, req.contract_address),
            "params": req.params
        }),
        gas_used: 1000,
        status: "success".to_string(),
    })
}

/// Get contract info endpoint
#[get("/contracts/{address}")]
async fn get_contract(
    data: web::Data<AppState>,
    address: web::Path<String>,
) -> impl Responder {
    let deployment_service = data.deployment_service.lock().unwrap();
    
    match deployment_service.get_contract(&address) {
        Some(contract) => HttpResponse::Ok().json(contract),
        None => HttpResponse::NotFound()
            .json(serde_json::json!({"error": "Contract not found"})),
    }
}

/// List all contracts endpoint
#[get("/contracts")]
async fn list_contracts(data: web::Data<AppState>) -> impl Responder {
    let deployment_service = data.deployment_service.lock().unwrap();
    let contracts = deployment_service.list_contracts();
    
    HttpResponse::Ok().json(contracts)
}

/// Deploy WASM contract via multipart upload
#[post("/deploy_wasm")]
async fn deploy_wasm_contract(
    data: web::Data<AppState>,
    mut payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    let mut wasm_bytes = Vec::new();
    
    // Process the multipart data to extract WASM file
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        if let Some(filename) = content_disposition.get_filename() {
            // Check if it's a WASM file
            if filename.ends_with(".wasm") {
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    wasm_bytes.extend_from_slice(&data);
                }
            }
        }
    }
    
    if wasm_bytes.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No WASM file provided"
/// Contract deployment service
pub struct DeploymentService {
    registry: ContractRegistry,
    blockchain: Blockchain,
}

impl DeploymentService {
    pub fn new() -> Self {
        Self {
            registry: ContractRegistry::new(),
            blockchain: Blockchain::new(),
        }
    }

    pub async fn deploy_from_file(
        &self,
        contract_path: &str,
        contract_type: &str,
        network: &str,
    ) -> Result<DeployResponse, String> {
        let contract_code = match contract_type {
            "solidity" => SolidityCompiler::compile(contract_path)?,
            "wasm" => {
                let wasm_bytes = fs::read(contract_path)
                    .map_err(|e| format!("Failed to read WASM file: {}", e))?;
                wasm_bytes
            }
            _ => return Err(format!("Unsupported contract type: {}", contract_type)),
        };

        match contract_type {
            "solidity" => self.registry.deploy_contract(contract_type, &contract_code, network).await,
            "wasm" => self.registry.deploy_wasm_contract(&contract_code, network).await,
            _ => Err(format!("Unsupported contract type: {}", contract_type)),
        }
    }

    pub async fn deploy_from_code(
        &self,
        contract_type: &str,
        contract_code: &[u8],
        network: &str,
    ) -> Result<DeployResponse, String> {
        match contract_type {
            "solidity" => self.registry.deploy_contract(contract_type, contract_code, network).await,
            "wasm" => self.registry.deploy_wasm_contract(contract_code, network).await,
            _ => Err(format!("Unsupported contract type: {}", contract_type)),
        }
    }

    pub fn get_contract(&self, address: &str) -> Option<DeployedContract> {
        self.registry.get_contract(address)
    }

    pub fn list_contracts(&self) -> Vec<DeployedContract> {
        self.registry.list_contracts()
    }
