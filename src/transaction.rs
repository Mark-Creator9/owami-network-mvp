use serde::{Serialize, Deserialize};
use ed25519_dalek::SigningKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub signature: Vec<u8>,
    pub hash: String,
}

impl Transaction {
    pub fn new_transfer(from: String, to: String, amount: u64, fee: u64, nonce: u64) -> Self {
        let mut tx = Self {
            from,
            to,
            amount,
            fee,
            nonce,
            signature: Vec::new(),
            hash: String::new(),
        };
        tx.hash = tx.calculate_hash();
        tx
    }

    pub fn sign(&mut self, _signing_key: &SigningKey) {
        // Implementation would go here
    }

    pub fn validate(&self) -> Result<(), String> {
        // Implementation would go here
        Ok(())
    }

    fn calculate_hash(&self) -> String {
        // Implementation would go here
        String::new()
    }
}