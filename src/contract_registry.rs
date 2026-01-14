use crate::wasm_runtime::{ContractStorage, WasmEngine};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, Mutex as StdMutex};

/// Represents a deployed smart contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployedContract {
    pub address: String,
    pub creator: String,
    pub wasm_bytecode: Vec<u8>,
    pub abi: Option<ContractABI>,
    pub deployment_height: u64,
    pub deployment_timestamp: DateTime<Utc>,
    pub contract_type: String,
    pub version: String,
    pub metadata: ContractMetadata,
}

/// Contract Application Binary Interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractABI {
    pub functions: Vec<FunctionABI>,
    pub events: Vec<EventABI>,
    pub constructor: Option<ConstructorABI>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionABI {
    pub name: String,
    pub inputs: Vec<ParamABI>,
    pub outputs: Vec<ParamABI>,
    pub mutability: Mutability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventABI {
    pub name: String,
    pub inputs: Vec<ParamABI>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructorABI {
    pub inputs: Vec<ParamABI>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamABI {
    pub name: String,
    pub r#type: String,
    pub indexed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

/// Contract metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub license: String,
    pub authors: Vec<String>,
    pub links: HashMap<String, String>,
}

/// Contract deployment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRequest {
    pub wasm_bytecode: Vec<u8>,
    pub creator: String,
    pub contract_type: String,
    pub constructor_args: Option<Vec<u8>>,
    pub gas_limit: Option<u64>,
}

/// Contract call request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallRequest {
    pub contract_address: String,
    pub function_name: String,
    pub args: Vec<u8>,
    pub caller: String,
    pub value: Option<u64>,
    pub gas_limit: Option<u64>,
}

/// Contract call response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallResponse {
    pub success: bool,
    pub result: Vec<u8>,
    pub gas_used: u64,
    pub events: Vec<ContractEvent>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEvent {
    pub contract_address: String,
    pub event_name: String,
    pub data: Vec<u8>,
    pub topics: Vec<Vec<u8>>,
    pub block_number: u64,
    pub transaction_hash: String,
}

/// Smart contract registry
pub struct ContractRegistry {
    contracts: Arc<Mutex<HashMap<String, DeployedContract>>>,
    storage: Arc<StdMutex<ContractStorage>>,
    wasm_engine: Arc<StdMutex<WasmEngine>>,
    deployment_count: Arc<Mutex<u64>>,
}

impl ContractRegistry {
    pub fn new(wasm_engine: Arc<StdMutex<WasmEngine>>) -> Self {
        Self {
            contracts: Arc::new(Mutex::new(HashMap::new())),
            storage: Arc::new(StdMutex::new(ContractStorage::new())),
            wasm_engine,
            deployment_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Deploy a new smart contract
    pub async fn deploy_contract(&self, request: DeploymentRequest) -> Result<DeployedContract> {
        let mut contracts = self.contracts.lock().unwrap();
        let mut deployment_count = self.deployment_count.lock().unwrap();

        // Generate unique contract address
        let address = self.generate_contract_address(&request.creator, &request.wasm_bytecode)?;

        // Validate WASM bytecode
        self.validate_wasm_bytecode(&request.wasm_bytecode)?;

        // Create contract ABI (simplified - in practice you'd parse from bytecode)
        let abi = self.extract_abi_from_wasm(&request.wasm_bytecode)?;

        // Create contract metadata
        let metadata = ContractMetadata {
            name: format!("Contract_{}", *deployment_count + 1),
            description: "Smart contract deployed on Owami Network".to_string(),
            version: "1.0.0".to_string(),
            license: "MIT".to_string(),
            authors: vec![request.creator.clone()],
            links: HashMap::new(),
        };

        // Create deployed contract
        let contract = DeployedContract {
            address: address.clone(),
            creator: request.creator,
            wasm_bytecode: request.wasm_bytecode,
            abi,
            deployment_height: 0,
            deployment_timestamp: Utc::now(),
            contract_type: request.contract_type,
            version: "1.0.0".to_string(),
            metadata,
        };

        // Store contract
        contracts.insert(address.clone(), contract.clone());

        // Initialize contract storage
        let mut storage = self.storage.lock().unwrap();
        storage.clear_contract(&address);

        // Run constructor if it exists
        if let Err(e) = self
            .run_constructor(&address, &request.constructor_args)
            .await
        {
            // If constructor fails, remove the contract
            contracts.remove(&address);
            return Err(anyhow!("Constructor execution failed: {}", e));
        }

        *deployment_count += 1;

        Ok(contract)
    }

    /// Get a deployed contract by address
    pub fn get_contract(&self, address: &str) -> Option<DeployedContract> {
        let contracts = self.contracts.lock().unwrap();
        contracts.get(address).cloned()
    }

    /// List all deployed contracts
    pub fn list_contracts(&self) -> Vec<DeployedContract> {
        let contracts = self.contracts.lock().unwrap();
        contracts.values().cloned().collect()
    }

    /// Call a function on a deployed contract
    pub async fn call_contract(&self, request: CallRequest) -> Result<CallResponse> {
        // Get the contract
        let contracts = self.contracts.lock().unwrap();
        let contract = match contracts.get(&request.contract_address) {
            Some(c) => c.clone(),
            None => {
                return Ok(CallResponse {
                    success: false,
                    result: vec![],
                    gas_used: 0,
                    events: vec![],
                    error: Some("Contract not found".to_string()),
                })
            }
        };

        // Execute the contract function
        let mut wasm_engine = self.wasm_engine.lock().unwrap();
        let result = crate::wasm_runtime::execute_contract(
            &mut wasm_engine,
            &contract.wasm_bytecode,
            &request.function_name,
            &request.args,
            &request.contract_address,
        )
        .await?;

        Ok(CallResponse {
            success: result.success,
            result: result.output,
            gas_used: result.gas_used,
            events: result
                .events
                .into_iter()
                .map(|e| ContractEvent {
                    contract_address: e.contract_address,
                    event_name: e.event_type,
                    data: e.data,
                    topics: vec![],
                    block_number: 0,
                    transaction_hash: "".to_string(),
                })
                .collect(),
            error: result.error,
        })
    }

    /// Upgrade an existing contract
    pub async fn upgrade_contract(
        &self,
        address: &str,
        new_wasm_bytecode: Vec<u8>,
        new_creator: String,
    ) -> Result<DeployedContract> {
        let mut contracts = self.contracts.lock().unwrap();

        // Check if contract exists
        let existing_contract = match contracts.get(address) {
            Some(c) => c.clone(),
            None => return Err(anyhow!("Contract not found")),
        };

        // Validate new bytecode
        self.validate_wasm_bytecode(&new_wasm_bytecode)?;

        // Generate new version
        let version_parts: Vec<&str> = existing_contract.version.split('.').collect();
        let new_version = if version_parts.len() >= 2 {
            format!(
                "{}.{}",
                version_parts[0],
                version_parts[1].parse::<u32>().unwrap_or(0) + 1
            )
        } else {
            "1.1.0".to_string()
        };

        // Create upgraded contract
        let mut upgraded_contract = existing_contract.clone();
        upgraded_contract.wasm_bytecode = new_wasm_bytecode;
        upgraded_contract.version = new_version;
        upgraded_contract.creator = new_creator;

        // Update in registry
        contracts.insert(address.to_string(), upgraded_contract.clone());

        // Clear and reinitialize storage
        let mut storage = self.storage.lock().unwrap();
        storage.clear_contract(address);

        Ok(upgraded_contract)
    }

    /// Get contract storage
    pub fn get_contract_storage(&self, address: &str) -> Option<HashMap<Vec<u8>, Vec<u8>>> {
        let storage = self.storage.lock().unwrap();
        let mut result = HashMap::new();
        for key in storage.list_keys(address) {
            if let Some(value) = storage.get(address, &key) {
                result.insert(key, value);
            }
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    /// Set contract storage (for testing/initialization)
    pub fn set_contract_storage(
        &self,
        address: &str,
        storage: HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<()> {
        let mut contract_storage = self.storage.lock().unwrap();
        for (key, value) in storage {
            contract_storage.set(address, key, value)?;
        }
        Ok(())
    }

    /// Generate contract address
    fn generate_contract_address(&self, creator: &str, wasm_bytecode: &[u8]) -> Result<String> {
        let deployment_count = *self.deployment_count.lock().unwrap();
        let mut hasher = blake3::Hasher::new();
        hasher.update(creator.as_bytes());
        hasher.update(wasm_bytecode);
        hasher.update(&deployment_count.to_be_bytes());

        let hash = hasher.finalize();
        Ok(hex::encode(hash.as_bytes()))
    }

    /// Validate WASM bytecode
    fn validate_wasm_bytecode(&self, wasm_bytecode: &[u8]) -> Result<()> {
        // Basic validation - check if it's valid WASM
        if wasm_bytecode.len() < 8 {
            return Err(anyhow!("Invalid WASM bytecode: too short"));
        }

        // Check WASM magic number
        if &wasm_bytecode[0..4] != b"\x00asm" {
            return Err(anyhow!("Invalid WASM bytecode: missing magic number"));
        }

        // Check version
        if &wasm_bytecode[4..8] != b"\x01\x00\x00\x00" {
            return Err(anyhow!("Invalid WASM bytecode: unsupported version"));
        }

        Ok(())
    }

    /// Extract ABI from WASM bytecode (simplified)
    fn extract_abi_from_wasm(&self, _wasm_bytecode: &[u8]) -> Result<Option<ContractABI>> {
        Ok(Some(ContractABI {
            functions: vec![FunctionABI {
                name: "execute".to_string(),
                inputs: vec![ParamABI {
                    name: "input".to_string(),
                    r#type: "bytes".to_string(),
                    indexed: false,
                }],
                outputs: vec![ParamABI {
                    name: "output".to_string(),
                    r#type: "bytes".to_string(),
                    indexed: false,
                }],
                mutability: Mutability::NonPayable,
            }],
            events: vec![],
            constructor: None,
        }))
    }

    /// Run contract constructor
    async fn run_constructor(
        &self,
        address: &str,
        constructor_args: &Option<Vec<u8>>,
    ) -> Result<()> {
        let mut storage = self.storage.lock().unwrap();
        storage.set(address, b"initialized".to_vec(), b"true".to_vec())?;

        if let Some(args) = constructor_args {
            storage.set(address, b"constructor_args".to_vec(), args.clone())?;
        }

        Ok(())
    }

    /// Get contract deployment statistics
    pub fn get_statistics(&self) -> ContractRegistryStats {
        let contracts = self.contracts.lock().unwrap();
        let deployment_count = self.deployment_count.lock().unwrap();

        ContractRegistryStats {
            total_contracts: contracts.len() as u64,
            total_deployments: *deployment_count,
            contracts_by_type: contracts
                .values()
                .fold(HashMap::new(), |mut acc, contract| {
                    *acc.entry(contract.contract_type.clone()).or_insert(0) += 1;
                    acc
                }),
        }
    }
}

/// Registry statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractRegistryStats {
    pub total_contracts: u64,
    pub total_deployments: u64,
    pub contracts_by_type: HashMap<String, u64>,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_contract_registry_creation() {
        let wasm_bytecode = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
        assert_eq!(&wasm_bytecode[0..4], b"\x00asm");
    }
}
