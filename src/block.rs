use serde::{Serialize, Deserialize};
use ed25519_dalek::{Signer, SigningKey};
use crate::Transaction;
use blake3;
use hex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub height: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        height: u64,
        previous_hash: String,
        transactions: Vec<Transaction>,
        validator_key: &SigningKey
    ) -> Self {
        let _rng = rand::thread_rng();
        let timestamp = chrono::Utc::now().timestamp() as u64;
        
        // Hash transactions
        let mut tx_hasher = blake3::Hasher::new();
        for tx in &transactions {
            tx_hasher.update(&tx.signature);
        }
        let tx_hash = tx_hasher.finalize().to_hex();
        
        // Create block data
        let block_data = format!(
            "{}{}{}{}",
            height,
            previous_hash,
            tx_hash,
            timestamp
        );
        
        // Validator signs the block
        let _signature = validator_key.sign(block_data.as_bytes());
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        let mut block = Block {
            header: BlockHeader {
                height,
                previous_hash,
                timestamp,
                signature: None,
            },
            transactions,
        };
        block.sign(validator_key);
        block
    }

    pub fn hash(&self) -> String {
        let header_json = serde_json::to_string(&self.header)
            .expect("Failed to serialize block header");
        let transactions_json = serde_json::to_string(&self.transactions)
            .expect("Failed to serialize transactions");

        let mut hasher = blake3::Hasher::new();
        hasher.update(header_json.as_bytes());
        hasher.update(transactions_json.as_bytes());
        hex::encode(hasher.finalize().as_bytes())
    }

    pub fn sign(&mut self, validator_key: &SigningKey) {
        let hash = self.hash();
        let signature = validator_key.sign(hash.as_bytes());
        self.header.signature = Some(hex::encode(signature.to_bytes().as_ref()));
    }
}