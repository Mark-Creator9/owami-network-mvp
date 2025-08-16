use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::Path;

#[derive(Deserialize)]
pub struct DeployRequest {
    pub contract_path: String,
    pub network: Option<String>, // e.g., "testnet"
}

#[derive(Serialize)]
pub struct DeployResponse {
    pub contract_address: String,
}

#[post("/deploy")]
pub async fn deploy_contract(req: web::Json<DeployRequest>) -> impl Responder {
    let network = req.network.clone().unwrap_or_else(|| "testnet".to_string());
    
    // Check if file exists
    if !Path::new(&req.contract_path).exists() {
        return HttpResponse::BadRequest().body(format!("Contract file {} does not exist", req.contract_path));
    }

    // Execute the owami-cli deploy command
    let output = Command::new("./owami-cli/target/debug/owami-cli")
        .arg("deploy")
        .arg("--file")
        .arg(&req.contract_path)
        .arg("--network")
        .arg(&network)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                // Parse the output to extract the contract address
                // For now, we'll simulate a contract address
                let contract_address = "0x1234567890abcdef1234567890abcdef12345678";
                HttpResponse::Ok().json(DeployResponse {
                    contract_address: contract_address.to_string(),
                })
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                HttpResponse::InternalServerError().body(format!("Deployment failed: {}", error_msg))
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to execute deployment: {}", e)),
    }
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
    let network = req.network.clone().unwrap_or_else(|| "testnet".to_string());
    
    // Convert params to JSON string if present
    let params_str = if let Some(ref params) = req.params {
        match serde_json::to_string(params) {
            Ok(s) => Some(s),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to serialize params: {}", e)),
        }
    } else {
        None
    };

    // Build command arguments
    let mut args = vec![
        "call",
        "--address",
        &req.contract_address,
        "--function",
        &req.function_name,
        "--network",
        &network,
    ];

    if let Some(params) = &params_str {
        args.push("--params");
        args.push(params);
    }

    // Execute the owami-cli call command
    let output = Command::new("./owami-cli/target/debug/owami-cli")
        .args(&args)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                HttpResponse::Ok().body(result.to_string())
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                HttpResponse::InternalServerError().body(format!("Call failed: {}", error_msg))
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to execute call: {}", e)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dapp")
            .service(deploy_contract)
            .service(call_contract)
    );
}
