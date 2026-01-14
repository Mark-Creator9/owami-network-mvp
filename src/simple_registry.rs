// Minimal working version focused on testnet functionality
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleContract {
    pub address: String,
    pub bytecode: Vec<u8>,
    pub creator: String,
    pub deployed_at: u64,
}

/// Simple in-memory contract registry for MVP
pub struct SimpleRegistry {
    contracts: HashMap<String, SimpleContract>,
}

impl SimpleRegistry {
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
        }
    }

    pub fn deploy_contract(&mut self, bytecode: Vec<u8>, creator: String) -> String {
        let hash = blake3::hash(&creator.as_bytes());
        let address = format!("0x{}", hex::encode(&hash.as_bytes()[..8]));
        let contract = SimpleContract {
            address: address.clone(),
            bytecode,
            creator,
            deployed_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        self.contracts.insert(address.clone(), contract);
        address
    }

    pub fn get_contract(&self, address: &str) -> Option<&SimpleContract> {
        self.contracts.get(address)
    }

    pub fn list_contracts(&self) -> Vec<SimpleContract> {
        self.contracts.values().cloned().collect()
    }
}
