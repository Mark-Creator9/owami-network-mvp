use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, VerifyingKey};
use blake3;
use crate::transaction::Transaction;
use crate::crypto_utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub height: u64,
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub signature: Vec<u8>,
}

impl Block {
    pub fn new(
        height: u64,
        previous_hash: String,
        transactions: Vec<Transaction>,
        signing_key: &SigningKey,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let merkle_root = Self::calculate_merkle_root(&transactions);
        
        let mut block = Block {
            header: BlockHeader {
                height,
                previous_hash,
                merkle_root,
                timestamp,
                nonce: 0,
            },
            transactions,
            signature: Vec::new(),
        };

        let signature = crypto_utils::sign_message(signing_key, block.hash().as_bytes());
        block.signature = crypto_utils::signature_to_bytes(&signature);
        
        block
    }

    fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return "0".repeat(64);
        }

        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| tx.hash())
            .collect();

        while hashes.len() > 1 {
            if hashes.len() % 2 != 0 {
                hashes.push(hashes.last().unwrap().clone());
            }

            let mut new_hashes = Vec::new();
            for chunk in hashes.chunks(2) {
                let combined = format!("{}{}", chunk[0], chunk[1]);
                let hash = blake3::hash(combined.as_bytes()).to_hex().to_string();
                new_hashes.push(hash);
            }
            hashes = new_hashes;
        }

        hashes[0].clone()
    }

    pub fn hash(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.header.height.to_le_bytes());
        hasher.update(self.header.previous_hash.as_bytes());
        hasher.update(self.header.merkle_root.as_bytes());
        hasher.update(&self.header.timestamp.to_le_bytes());
        hasher.update(&self.header.nonce.to_le_bytes());
        
        for tx in &self.transactions {
            hasher.update(tx.hash().as_bytes());
        }
        
        hasher.finalize().to_hex().to_string()
    }

    pub fn verify_signature(&self, public_key: &VerifyingKey) -> bool {
        let signature = match crypto_utils::signature_from_bytes(&self.signature) {
            Ok(sig) => sig,
            Err(_) => return false,
        };
        
        crypto_utils::verify_signature(public_key, self.hash().as_bytes(), &signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use crate::transaction::Transaction;

    #[test]
    fn test_block_creation() {
        let signing_key = crypto_utils::default_signing_key();
        let public_key = signing_key.verifying_key();
        
        let from = hex::encode(public_key.to_bytes());
        let tx = Transaction::new(
            from.clone(),
            "recipient".to_string(),
            100,
            None,
            &signing_key,
        );
        
        let block = Block::new(
            1,
            "0".repeat(64),
            vec![tx],
            &signing_key,
        );
        
        assert_eq!(block.header.height, 1);
        assert_eq!(block.header.previous_hash, "0".repeat(64));
        assert!(block.verify_signature(&public_key));
    }

    #[test]
    fn test_merkle_root_calculation() {
        let signing_key = crypto_utils::default_signing_key();
        
        let tx1 = Transaction::new(
            "from1".to_string(),
            "to1".to_string(),
            100,
            None,
            &signing_key,
        );
        
        let tx2 = Transaction::new(
            "from2".to_string(),
            "to2".to_string(),
            200,
            None,
            &signing_key,
        );
        
        let root = Block::calculate_merkle_root(&[tx1, tx2]);
        assert_eq!(root.len(), 64);
    }
}