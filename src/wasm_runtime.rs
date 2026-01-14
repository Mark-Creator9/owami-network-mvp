use crate::blockchain::Blockchain;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex as StdMutex};
use wasmtime::*;

/// Host functions available to WASM contracts
#[derive(Clone)]
pub struct HostFunctions {
    pub blockchain: Arc<Blockchain>,
    pub storage: Arc<StdMutex<ContractStorage>>,
    pub gas_meter: Arc<StdMutex<GasMeter>>,
}

impl HostFunctions {
    pub fn new(blockchain: Arc<Blockchain>, storage: Arc<StdMutex<ContractStorage>>) -> Self {
        Self {
            blockchain,
            storage,
            gas_meter: Arc::new(StdMutex::new(GasMeter::new())),
        }
    }

    /// Get storage value for a key
    pub fn get_storage(&self, contract_address: &str, key: &[u8]) -> Option<Vec<u8>> {
        let storage = self.storage.lock().unwrap();
        storage.get(contract_address, key)
    }

    /// Set storage value for a key
    pub fn set_storage(&self, contract_address: &str, key: &[u8], value: &[u8]) -> Result<()> {
        let mut storage = self.storage.lock().unwrap();
        storage.set(contract_address, key.to_vec(), value.to_vec())?;
        let mut gas_meter = self.gas_meter.lock().unwrap();
        gas_meter.record_storage_write(key.len() + value.len())?;
        Ok(())
    }

    /// Get balance for an address
    pub fn get_balance(&self, address: &str) -> u64 {
        self.blockchain.get_balance(address)
    }

    /// Transfer tokens between addresses
    pub fn transfer(&self, _from: &str, _to: &str, _amount: u64) -> Result<()> {
        let mut gas_meter = self.gas_meter.lock().unwrap();
        gas_meter.record_transfer()?;
        // This would be implemented as a transaction in the blockchain
        Ok(())
    }

    /// Emit an event from contract execution
    pub fn emit_event(
        &self,
        _contract_address: &str,
        _event_type: &str,
        data: &[u8],
    ) -> Result<()> {
        let mut gas_meter = self.gas_meter.lock().unwrap();
        gas_meter.record_event()?;
        // Log the event for external listeners
        println!(
            "Event from {}: {} - {:?}",
            _contract_address,
            _event_type,
            hex::encode(data)
        );
        Ok(())
    }

    /// Get current block height
    pub fn get_block_height(&self) -> u64 {
        self.blockchain.get_block_height()
    }

    /// Get current timestamp
    pub fn get_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Gas metering for WASM contract execution
#[derive(Clone)]
pub struct GasMeter {
    pub limit: u64,
    pub used: u64,
}

impl GasMeter {
    pub fn new() -> Self {
        Self {
            limit: 1_000_000, // 1M gas limit per execution
            used: 0,
        }
    }

    pub fn set_limit(&mut self, limit: u64) {
        self.limit = limit;
        self.used = 0;
    }

    pub fn record_instruction(&mut self) -> Result<()> {
        self.used += 1;
        self.check_limit()
    }

    pub fn record_memory_allocation(&mut self, bytes: usize) -> Result<()> {
        self.used += (bytes / 8) as u64; // 1 gas per 8 bytes
        self.check_limit()
    }

    pub fn record_storage_read(&mut self, size: usize) -> Result<()> {
        self.used += (size / 32) as u64; // 1 gas per 32 bytes
        self.check_limit()
    }

    pub fn record_storage_write(&mut self, size: usize) -> Result<()> {
        self.used += (size / 16) as u64; // 1 gas per 16 bytes
        self.check_limit()
    }

    pub fn record_transfer(&mut self) -> Result<()> {
        self.used += 100; // Fixed cost for transfers
        self.check_limit()
    }

    pub fn record_event(&mut self) -> Result<()> {
        self.used += 50; // Fixed cost for events
        self.check_limit()
    }

    fn check_limit(&self) -> Result<()> {
        if self.used > self.limit {
            Err(anyhow!(
                "Gas limit exceeded: {} > {}",
                self.used,
                self.limit
            ))
        } else {
            Ok(())
        }
    }

    pub fn remaining(&self) -> u64 {
        self.limit.saturating_sub(self.used)
    }
}

/// Contract storage interface
#[derive(Clone)]
pub struct ContractStorage {
    storage: HashMap<String, HashMap<Vec<u8>, Vec<u8>>>,
}

impl ContractStorage {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn get(&self, contract_address: &str, key: &[u8]) -> Option<Vec<u8>> {
        self.storage
            .get(contract_address)
            .and_then(|contract_storage| contract_storage.get(key))
            .cloned()
    }

    pub fn set(&mut self, contract_address: &str, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        self.storage
            .entry(contract_address.to_string())
            .or_insert_with(HashMap::new)
            .insert(key, value);
        Ok(())
    }

    pub fn remove(&mut self, contract_address: &str, key: &[u8]) -> Option<Vec<u8>> {
        self.storage
            .get_mut(contract_address)
            .and_then(|contract_storage| contract_storage.remove(key))
    }

    pub fn list_keys(&self, contract_address: &str) -> Vec<Vec<u8>> {
        self.storage
            .get(contract_address)
            .map(|storage| storage.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn clear_contract(&mut self, contract_address: &str) {
        self.storage.remove(contract_address);
    }
}

/// WASM execution engine
pub struct WasmEngine {
    engine: Engine,
    store: Store<()>,
    host_functions: Arc<HostFunctions>,
}

impl WasmEngine {
    pub fn new(
        blockchain: Arc<Blockchain>,
        storage: Arc<StdMutex<ContractStorage>>,
    ) -> Result<Self> {
        let engine = Engine::default();
        let store = Store::new(&engine, ());
        let host_functions = HostFunctions::new(blockchain, storage);

        Ok(Self {
            engine,
            store,
            host_functions: Arc::new(host_functions),
        })
    }

    /// Load and instantiate a WASM module
    pub async fn load_module(&mut self, wasm_bytes: &[u8]) -> Result<WasmModule> {
        // Create module from WASM bytes
        let module = Module::from_binary(&self.engine, wasm_bytes)?;

        // Create linker with host functions
        let linker = Linker::new(&self.engine);

        // Instantiate the module
        let instance = linker.instantiate(&mut self.store, &module)?;

        Ok(WasmModule { module, instance })
    }

    /// Execute a WASM function with given arguments
    pub async fn execute_function(
        &mut self,
        wasm_module: &WasmModule,
        function_name: &str,
        args: &[wasmtime::Val],
    ) -> Result<Vec<wasmtime::Val>> {
        let func = wasm_module
            .instance
            .get_func(&mut self.store, function_name)
            .ok_or_else(|| anyhow!("Function {} not found", function_name))?;

        // Allocate space for return values
        let mut returns = vec![wasmtime::Val::I32(0); 1];

        // Execute with gas metering
        let _result = func.call_async(&mut self.store, args, &mut returns).await?;

        // Convert wasmtime values to Vec
        let values: Vec<wasmtime::Val> = returns.into_iter().collect();

        Ok(values)
    }
}

/// Represents a loaded WASM module
pub struct WasmModule {
    pub module: Module,
    pub instance: Instance,
}

/// WASM contract execution result
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: Vec<u8>,
    pub gas_used: u64,
    pub events: Vec<ContractEvent>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractEvent {
    pub contract_address: String,
    pub event_type: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

/// Execute a WASM contract with given input
pub async fn execute_contract(
    engine: &mut WasmEngine,
    wasm_bytes: &[u8],
    function_name: &str,
    input: &[u8],
    contract_address: &str,
) -> Result<ExecutionResult> {
    // Load the module
    let wasm_module = engine.load_module(wasm_bytes).await?;

    // Prepare arguments (simplified - using i32 for pointers and lengths)
    let args = vec![
        Val::I32(contract_address.len() as i32),
        Val::I32(input.len() as i32),
    ];

    // Execute the function
    let result = engine
        .execute_function(&wasm_module, function_name, &args)
        .await;

    match result {
        Ok(output) => {
            let gas_meter = engine.host_functions.gas_meter.lock().unwrap();
            let gas_used = gas_meter.used;
            // For now, return empty output - in practice you'd handle memory properly
            Ok(ExecutionResult {
                success: true,
                output: output
                    .get(0)
                    .map(|v| v.unwrap_i32().to_le_bytes().to_vec())
                    .unwrap_or_default(),
                gas_used,
                events: vec![], // Would collect events during execution
                error: None,
            })
        }
        Err(e) => {
            let gas_meter = engine.host_functions.gas_meter.lock().unwrap();
            let gas_used = gas_meter.used;
            Ok(ExecutionResult {
                success: false,
                output: vec![],
                gas_used,
                events: vec![],
                error: Some(e.to_string()),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::Blockchain;
    use crate::config::AppConfig;

    #[tokio::test]
    async fn test_wasm_engine_creation() -> Result<()> {
        let config = AppConfig::load().unwrap();
        let blockchain = Arc::new(Blockchain::new(&config));
        let storage = Arc::new(StdMutex::new(ContractStorage::new()));

        let _engine = WasmEngine::new(blockchain, storage)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_contract_storage() -> Result<()> {
        let mut storage = ContractStorage::new();

        // Test basic storage operations
        storage.set("contract1", b"key1".to_vec(), b"value1".to_vec())?;
        assert_eq!(storage.get("contract1", b"key1"), Some(b"value1".to_vec()));

        storage.remove("contract1", b"key1");
        assert_eq!(storage.get("contract1", b"key1"), None);

        Ok(())
    }

    #[tokio::test]
    async fn test_gas_metering() -> Result<()> {
        let mut gas_meter = GasMeter::new();
        gas_meter.set_limit(1000);

        // Record some operations
        gas_meter.record_instruction()?;
        gas_meter.record_memory_allocation(64)?;
        gas_meter.record_storage_read(32)?;
        gas_meter.record_storage_write(16)?;

        assert_eq!(gas_meter.used, 1 + 8 + 1 + 1);
        assert_eq!(gas_meter.remaining(), 1000 - 11);

        // Should fail when exceeding limit
        gas_meter.set_limit(10);
        assert!(gas_meter.record_instruction().is_err());

        Ok(())
    }
}
