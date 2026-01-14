use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Response,
};
use base64::Engine as _;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex as StdMutex};

// Import our modules
use crate::{
    blockchain::Blockchain,
    compiler::CompilationService,
    config::AppConfig,
    contract_registry::{
        CallRequest, CallResponse, ContractRegistry, DeployedContract, DeploymentRequest,
    },
    wasm_runtime::{ContractStorage, WasmEngine},
};

/// Global deployment service
#[derive(Clone)]
pub struct AppState {
    pub deployment_service: Arc<StdMutex<DeploymentService>>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            deployment_service: Arc::new(StdMutex::new(DeploymentService::new(config))),
        }
    }
}

/// Contract deployment service
#[allow(dead_code)]
pub struct DeploymentService {
    registry: ContractRegistry,
    wasm_engine: Arc<StdMutex<WasmEngine>>,
    compilation_service: CompilationService,
    blockchain: Arc<Blockchain>,
}

impl DeploymentService {
    pub fn new(config: AppConfig) -> Self {
        let blockchain = Arc::new(Blockchain::new(&config));
        let storage = Arc::new(StdMutex::new(ContractStorage::new()));
        let wasm_engine = Arc::new(StdMutex::new(
            WasmEngine::new(blockchain.clone(), storage).unwrap(),
        ));
        let registry = ContractRegistry::new(wasm_engine.clone());
        let compilation_service =
            CompilationService::new().unwrap_or_else(|_| CompilationService::new().unwrap());

        Self {
            registry,
            wasm_engine,
            compilation_service,
            blockchain,
        }
    }

    /// Deploy a new smart contract from WASM bytes
    pub async fn deploy_wasm_contract(
        &mut self,
        wasm_bytes: Vec<u8>,
        creator: String,
        contract_type: String,
    ) -> Result<DeployedContract, String> {
        let request = DeploymentRequest {
            wasm_bytecode: wasm_bytes,
            creator,
            contract_type,
            constructor_args: None,
            gas_limit: Some(1_000_000),
        };

        self.registry
            .deploy_contract(request)
            .await
            .map_err(|e| e.to_string())
    }

    /// Deploy a contract from source code
    pub async fn deploy_from_source(
        &mut self,
        source_code: String,
        language: String,
        creator: String,
        contract_type: String,
    ) -> Result<DeployedContract, String> {
        let wasm_bytes = match language.to_lowercase().as_str() {
            "rust" | "wasm" => self
                .compilation_service
                .compile_rust_to_wasm(&source_code, None)
                .map_err(|e| e.to_string())?,
            "solidity" => self
                .compilation_service
                .compile_solidity_to_wasm(&source_code, None)
                .map_err(|e| e.to_string())?,
            _ => return Err("Unsupported language. Use 'rust' or 'solidity'".to_string()),
        };

        self.deploy_wasm_contract(wasm_bytes, creator, contract_type)
            .await
    }

    /// Call a function on a deployed contract
    pub async fn call_contract(
        &mut self,
        contract_address: String,
        function_name: String,
        args: Vec<u8>,
        caller: String,
    ) -> Result<CallResponse, String> {
        let request = CallRequest {
            contract_address,
            function_name,
            args,
            caller,
            value: None,
            gas_limit: Some(1_000_000),
        };

        self.registry
            .call_contract(request)
            .await
            .map_err(|e| e.to_string())
    }
}

/// Request to deploy a WASM contract
#[derive(Debug, Deserialize)]
pub struct WasmContractRequest {
    pub wasm_bytecode: String,
    pub creator: String,
    pub contract_type: String,
}

/// Request to deploy from source code
#[derive(Debug, Deserialize)]
pub struct SourceContractRequest {
    pub source_code: String,
    pub language: String,
    pub creator: String,
    pub contract_type: String,
}

/// Request to call a contract function
#[derive(Debug, Deserialize)]
pub struct ContractCallRequest {
    pub contract_address: String,
    pub function_name: String,
    pub args: Vec<u8>,
    pub caller: String,
}

/// Response for contract deployment
#[derive(Debug, Serialize)]
pub struct DeploymentResponse {
    pub success: bool,
    pub contract_address: String,
    pub message: String,
    pub contract: Option<DeployedContract>,
}

/// Response for contract call
#[derive(Debug, Serialize)]
pub struct ContractCallResponse {
    pub success: bool,
    pub result: Vec<u8>,
    pub gas_used: u64,
    pub error: Option<String>,
}

/// API Routes

/// Upload contract file
pub async fn upload_contract(
    State(state): State<AppState>,
) -> Result<Response<String>, (StatusCode, String)> {
    // Simplified file upload - in practice you'd handle multipart form data
    let service = state.deployment_service.lock().unwrap();
    let stats = service.registry.get_statistics();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&stats).unwrap())
        .unwrap())
}

/// Deploy WASM contract
pub async fn deploy_wasm_contract(
    State(state): State<AppState>,
    Json(request): Json<WasmContractRequest>,
) -> Result<Response<String>, (StatusCode, String)> {
    let mut service = state.deployment_service.lock().unwrap();

    // Decode base64 WASM bytecode
    let wasm_bytes = base64::engine::general_purpose::STANDARD
        .decode(&request.wasm_bytecode)
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid base64 WASM bytecode".to_string(),
            )
        })?;

    match service
        .deploy_wasm_contract(wasm_bytes, request.creator, request.contract_type)
        .await
    {
        Ok(contract) => {
            let response = DeploymentResponse {
                success: true,
                contract_address: contract.address.clone(),
                message: "Contract deployed successfully".to_string(),
                contract: Some(contract),
            };

            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(serde_json::to_string(&response).unwrap())
                .unwrap())
        }
        Err(e) => {
            let response = DeploymentResponse {
                success: false,
                contract_address: String::new(),
                message: e,
                contract: None,
            };

            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(serde_json::to_string(&response).unwrap())
                .unwrap())
        }
    }
}

/// Deploy contract from source
pub async fn deploy_contract(
    State(state): State<AppState>,
    Json(request): Json<SourceContractRequest>,
) -> Result<Response<String>, (StatusCode, String)> {
    let mut service = state.deployment_service.lock().unwrap();

    match service
        .deploy_from_source(
            request.source_code,
            request.language,
            request.creator,
            request.contract_type,
        )
        .await
    {
        Ok(contract) => {
            let response = DeploymentResponse {
                success: true,
                contract_address: contract.address.clone(),
                message: "Contract deployed successfully".to_string(),
                contract: Some(contract),
            };

            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(serde_json::to_string(&response).unwrap())
                .unwrap())
        }
        Err(e) => {
            let response = DeploymentResponse {
                success: false,
                contract_address: String::new(),
                message: e,
                contract: None,
            };

            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(serde_json::to_string(&response).unwrap())
                .unwrap())
        }
    }
}

/// Call contract function
pub async fn call_contract(
    State(state): State<AppState>,
    Json(request): Json<ContractCallRequest>,
) -> Result<Response<String>, (StatusCode, String)> {
    let mut service = state.deployment_service.lock().unwrap();

    match service
        .call_contract(
            request.contract_address,
            request.function_name,
            request.args,
            request.caller,
        )
        .await
    {
        Ok(response) => {
            let contract_response = ContractCallResponse {
                success: response.success,
                result: response.result,
                gas_used: response.gas_used,
                error: response.error,
            };

            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(serde_json::to_string(&contract_response).unwrap())
                .unwrap())
        }
        Err(e) => {
            let response = ContractCallResponse {
                success: false,
                result: vec![],
                gas_used: 0,
                error: Some(e),
            };

            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(serde_json::to_string(&response).unwrap())
                .unwrap())
        }
    }
}

/// Deploy contract from file
pub async fn deploy_contract_file(
    State(state): State<AppState>,
) -> Result<Response<String>, (StatusCode, String)> {
    // Simplified - in practice you'd handle file uploads
    let service = state.deployment_service.lock().unwrap();
    let stats = service.registry.get_statistics();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&stats).unwrap())
        .unwrap())
}

/// Get contract by address
pub async fn get_contract(
    State(state): State<AppState>,
    address: String,
) -> Result<Response<String>, (StatusCode, String)> {
    let service = state.deployment_service.lock().unwrap();

    if let Some(contract) = service.registry.get_contract(&address) {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::to_string(&contract).unwrap())
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Contract not found".to_string())
            .unwrap())
    }
}

/// List all contracts
pub async fn list_contracts(
    State(state): State<AppState>,
) -> Result<Response<String>, (StatusCode, String)> {
    let service = state.deployment_service.lock().unwrap();
    let contracts = service.registry.list_contracts();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&contracts).unwrap())
        .unwrap())
}

/// Get contract storage
pub async fn get_contract_storage(
    State(state): State<AppState>,
    address: String,
) -> Result<Response<String>, (StatusCode, String)> {
    let service = state.deployment_service.lock().unwrap();

    if let Some(storage) = service.registry.get_contract_storage(&address) {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::to_string(&storage).unwrap())
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Storage not found".to_string())
            .unwrap())
    }
}

/// Get deployment statistics
pub async fn get_deployment_stats(
    State(state): State<AppState>,
) -> Result<Response<String>, (StatusCode, String)> {
    let service = state.deployment_service.lock().unwrap();
    let stats = service.registry.get_statistics();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&stats).unwrap())
        .unwrap())
}

/// Compile source code
pub async fn compile_contract(
    State(state): State<AppState>,
    Json(request): Json<SourceContractRequest>,
) -> Result<Response<String>, (StatusCode, String)> {
    let service = state.deployment_service.lock().unwrap();

    let result = match request.language.to_lowercase().as_str() {
        "rust" | "wasm" => service
            .compilation_service
            .compile_rust_to_wasm(&request.source_code, None),
        "solidity" => service
            .compilation_service
            .compile_solidity_to_wasm(&request.source_code, None),
        _ => return Err((StatusCode::BAD_REQUEST, "Unsupported language".to_string())),
    };

    match result {
        Ok(wasm_bytes) => {
            let wasm_base64 = base64::engine::general_purpose::STANDARD.encode(&wasm_bytes);
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(
                    serde_json::to_string(&serde_json::json!({
                        "success": true,
                        "wasm_bytecode": wasm_base64,
                        "size": wasm_bytes.len()
                    }))
                    .unwrap(),
                )
                .unwrap())
        }
        Err(e) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(
                serde_json::to_string(&serde_json::json!({
                    "success": false,
                    "error": e.to_string()
                }))
                .unwrap(),
            )
            .unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;

    #[tokio::test]
    async fn test_deployment_service_creation() {
        let config = AppConfig::load().unwrap();
        let _service = DeploymentService::new(config);
    }
}
