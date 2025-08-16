use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

// Import our WASM module
use crate::wasm::WasmExecutor;

#[derive(Deserialize)]
pub struct DeployRequest {
    pub contract_path: String,
    pub network: Option<String>, // e.g., "testnet"
}

#[derive(Serialize)]
pub struct DeployResponse {
    pub contract_address: String,
}

#[post("/upload")]
async fn upload_contract(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    let mut filepath = String::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        if let Some(filename) = content_disposition.get_filename() {
            let sanitized_filename = sanitize_filename::sanitize(filename);
            let filepath_str = format!("./uploads/{}", sanitized_filename);
            filepath = filepath_str.clone();
            
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
    Ok(HttpResponse::Ok().body(filepath))
}

#[post("/deploy")]
pub async fn deploy_contract(req: web::Json<DeployRequest>) -> impl Responder {
    let _network = req.network.clone().unwrap_or_else(|| "testnet".to_string());

    // Check if file exists
    if !Path::new(&req.contract_path).exists() {
        return HttpResponse::BadRequest()
            .body(format!("Contract file {} does not exist", req.contract_path));
    }

    // Simulate contract deployment
    let contract_address = "0x1234567890abcdef1234567890abcdef12345678";
    HttpResponse::Ok().json(DeployResponse {
        contract_address: contract_address.to_string(),
    })
}

#[derive(Deserialize)]
pub struct CallRequest {
    pub contract_address: String,
    pub function_name: String,
    pub params: Option<serde_json::Value>,
    pub network: Option<String>,
}

#[post("/call")]
pub async fn call_contract(req: web::Json<CallRequest>) -> impl Responder {
    let _network = req.network.clone().unwrap_or_else(|| "testnet".to_string());
    
    // Simulate contract call
    HttpResponse::Ok().body(format!("Called function {} on contract {} with params {:?}", 
        req.function_name, req.contract_address, req.params))
}

#[post("/deploy_wasm")]
async fn deploy_wasm_contract(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
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
        return Ok(HttpResponse::BadRequest().body("No WASM file provided"));
    }
    
    // Deploy the WASM contract
    match WasmExecutor::new() {
        Ok(executor) => {
            match executor.deploy_contract(&wasm_bytes) {
                Ok(contract_address) => {
                    Ok(HttpResponse::Ok().json(serde_json::json!({
                        "contract_address": contract_address,
                        "status": "deployed"
                    })))
                },
                Err(e) => {
                    Ok(HttpResponse::InternalServerError().body(format!("Deployment failed: {}", e)))
                }
            }
        },
        Err(e) => {
            Ok(HttpResponse::InternalServerError().body(format!("Failed to create WASM executor: {}", e)))
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dapp")
            .service(upload_contract)
            .service(deploy_contract)
            .service(call_contract)
            .service(deploy_wasm_contract)  // Add the new WASM deployment endpoint
    );
}