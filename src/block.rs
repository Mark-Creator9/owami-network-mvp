use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Verifier};
use blake3;
use crate::transaction::Transaction;

#[cfg(test)]
use crate::crypto_utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub height: u64,
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub nonce: u64,
    pub producer: String,
    pub signature: String,
}

impl std::fmt::Display for BlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}",
            self.height,
            self.previous_hash,
            self.merkle_root,
            self.timestamp,
            self.nonce,
            self.producer
        )
    }
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
                producer: hex::encode(signing_key.verifying_key().to_bytes()),
                signature: String::new(), // Will be filled after signing
            },
            transactions,
            signature: Vec::new(),
        };
        
        // Sign the block
        let message = block.hash_data();
        let signature = signing_key.sign(&message);
        block.signature = signature.to_bytes().to_vec();
        block.header.signature = hex::encode(signature.to_bytes());
        
        block
    }
    
    pub fn hash_data(&self) -> Vec<u8> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.header.height.to_be_bytes());
        hasher.update(self.header.previous_hash.as_bytes());
        hasher.update(self.header.merkle_root.as_bytes());
        hasher.update(&self.header.timestamp.to_be_bytes());
        hasher.update(&self.header.nonce.to_be_bytes());
        hasher.update(self.header.producer.as_bytes());
        
        for tx in &self.transactions {
            hasher.update(tx.hash().as_bytes());
        }
        
        hasher.finalize().as_bytes().to_vec()
    }
    
    pub fn verify_signature(&self, public_key: &VerifyingKey) -> bool {
        let message = self.hash_data();
        let signature_bytes = match hex::decode(&self.header.signature) {
            Ok(bytes) => {
                if bytes.len() != 64 {
                    return false;
                }
                let mut sig_bytes = [0u8; 64];
                sig_bytes.copy_from_slice(&bytes);
                sig_bytes
            }
            Err(_) => return false,
        };
        let signature = ed25519_dalek::Signature::from_bytes(&signature_bytes);
        
        public_key.verify(&message, &signature).is_ok()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;

    #[test]
    fn test_block_creation() -> Result<(), Box<dyn std::error::Error>> {
        let signing_key = crypto_utils::default_signing_key();
        let public_key = signing_key.verifying_key();
        
        let from = hex::encode(public_key.to_bytes());
        let mut tx = Transaction::new(
            from.clone(),
            "recipient".to_string(),
            100,
            None,
        );
        tx.sign(&signing_key)?;
        
        let block = Block::new(
            1,
            "0".repeat(64),
            vec![tx],
            &signing_key,
        );
        
        assert_eq!(block.header.height, 1);
        assert_eq!(block.header.previous_hash, "0".repeat(64));
        assert!(block.verify_signature(&public_key));
        Ok(())
    }

    #[test]
    fn test_merkle_root_calculation() -> Result<(), Box<dyn std::error::Error>> {
        let signing_key = crypto_utils::default_signing_key();
        
        let mut tx1 = Transaction::new(
            "from1".to_string(),
            "to1".to_string(),
            100,
            None,
        );
        tx1.sign(&signing_key)?;
        
        let mut tx2 = Transaction::new(
            "from2".to_string(),
            "to2".to_string(),
            200,
            None,
        );
        tx2.sign(&signing_key)?;
        
        let root = Block::calculate_merkle_root(&[tx1, tx2]);
        assert_eq!(root.len(), 64);
        Ok(())
    }

    #[test]
    fn test_create_block_with_valid_transactions() -> Result<(), Box<dyn std::error::Error>> {
        let signing_key = crypto_utils::default_signing_key();
        let public_key = signing_key.verifying_key();
        
        // Create multiple valid transactions
        let from = hex::encode(public_key.to_bytes());
        let mut tx1 = Transaction::new(
            from.clone(),
            "recipient1".to_string(),
            50,
            Some("test_data".to_string()),
        );
        tx1.sign(&signing_key)?;
        
        let mut tx2 = Transaction::new(
            from.clone(),
            "recipient2".to_string(),
            75,
            None,
        );
        tx2.sign(&signing_key)?;
        
        let transactions = vec![tx1, tx2];
        let block = Block::new(
            42,
            "previous_block_hash".to_string(),
            transactions.clone(),
            &signing_key,
        );
        
        assert_eq!(block.header.height, 42);
        assert_eq!(block.header.previous_hash, "previous_block_hash");
        assert_eq!(block.transactions.len(), 2);
        assert_eq!(block.header.producer, hex::encode(public_key.to_bytes()));
        assert!(block.header.timestamp > 0);
        assert!(!block.signature.is_empty());
        Ok(())
    }

    #[test]
    fn test_verify_block_signature_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let signing_key = crypto_utils::default_signing_key();
        let public_key = signing_key.verifying_key();
        
        let from = hex::encode(public_key.to_bytes());
        let mut tx = Transaction::new(
            from,
            "recipient".to_string(),
            100,
            None,
        );
        tx.sign(&signing_key)?;
        
        let block = Block::new(
            1,
            "0".repeat(64),
            vec![tx],
            &signing_key,
        );
        
        // Verify with correct public key
        assert!(block.verify_signature(&public_key));
        
        // Verify with different public key should fail
        let (_, wrong_public_key) = crypto_utils::generate_keypair();
        assert!(!block.verify_signature(&wrong_public_key));
        
        Ok(())
    }

    #[test]
    fn test_hash_block_with_consistent_output() -> Result<(), Box<dyn std::error::Error>> {
        let signing_key = crypto_utils::default_signing_key();
        let public_key = signing_key.verifying_key();
        
        let from = hex::encode(public_key.to_bytes());
        let mut tx = Transaction::new(
            from,
            "recipient".to_string(),
            100,
            None,
        );
        tx.sign(&signing_key)?;
        
        let block1 = Block::new(
            1,
            "0".repeat(64),
            vec![tx.clone()],
            &signing_key,
        );
        
        let block2 = Block::new(
            1,
            "0".repeat(64),
            vec![tx],
            &signing_key,
        );
        
        // Note: Hashes will be different due to different timestamps
        // But the hash format should be consistent
        let hash1 = block1.hash();
        let hash2 = block2.hash();
        
        assert_eq!(hash1.len(), 64); // blake3 hex output length
        assert_eq!(hash2.len(), 64);
        assert!(hash1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(hash2.chars().all(|c| c.is_ascii_hexdigit()));
        
        Ok(())
    }

    #[test]
    fn test_empty_transactions_merkle_root() {
        let root = Block::calculate_merkle_root(&[]);
        assert_eq!(root, "0".repeat(64));
        assert_eq!(root.len(), 64);
    }

    #[test]
    fn test_single_transaction_merkle_root() -> Result<(), Box<dyn std::error::Error>> {
        let signing_key = crypto_utils::default_signing_key();
        let public_key = signing_key.verifying_key();
        
        let from = hex::encode(public_key.to_bytes());
        let mut tx = Transaction::new(
            from,
            "recipient".to_string(),
            100,
            None,
        );
        tx.sign(&signing_key)?;
        
        let root = Block::calculate_merkle_root(&[tx.clone()]);
        
        // For a single transaction, merkle root should be the transaction hash
        assert_eq!(root, tx.hash());
        assert_eq!(root.len(), 64);
        
        Ok(())
    }

    #[test]
    fn test_invalid_signature_verification_fails() -> Result<(), Box<dyn std::error::Error>> {
        let signing_key = crypto_utils::default_signing_key();
        let public_key = signing_key.verifying_key();
        
        let from = hex::encode(public_key.to_bytes());
        let mut tx = Transaction::new(
            from,
            "recipient".to_string(),
            100,
            None,
        );
        tx.sign(&signing_key)?;
        
        let mut block = Block::new(
            1,
            "0".repeat(64),
            vec![tx],
            &signing_key,
        );
        
        // Corrupt the signature
        block.signature = vec![0u8; 64];
        
        // Verification should fail
        assert!(!block.verify_signature(&public_key));
        
        // Test with empty signature
        block.signature = vec![];
        assert!(!block.verify_signature(&public_key));
        
        // Test with wrong signature length
        block.signature = vec![0u8; 32];
        assert!(!block.verify_signature(&public_key));
        
        Ok(())
    }

    #[test]
    fn test_block_hash_deterministic_output() -> Result<(), Box<dyn std::error::Error>> {
        let signing_key = crypto_utils::default_signing_key();
        let public_key = signing_key.verifying_key();
        
        let from = hex::encode(public_key.to_bytes());
        let mut tx = Transaction::new(
            from,
            "recipient".to_string(),
            100,
            None,
        );
        tx.sign(&signing_key)?;
        
        // Create block with fixed timestamp for deterministic testing
        let mut block = Block {
            header: BlockHeader {
                height: 1,
                previous_hash: "0".repeat(64),
                merkle_root: Block::calculate_merkle_root(&[tx.clone()]),
                timestamp: 1234567890,
                nonce: 42,
                producer: hex::encode(public_key.to_bytes()),
                signature: hex::encode(&[1, 2, 3, 4]),
            },
            transactions: vec![tx],
            signature: vec![1, 2, 3, 4], // dummy signature for testing
        };
        
        let hash1 = block.hash();
        let hash2 = block.hash();
        
        // Same block should produce same hash
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64);
        
        // Changing nonce should change hash
        block.header.nonce = 43;
        let hash3 = block.hash();
        assert_ne!(hash1, hash3);
        
        Ok(())
    }

    #[test]
    fn test_hash_data_vs_hash_consistency() -> Result<(), Box<dyn std::error::Error>> {
        let signing_key = crypto_utils::default_signing_key();
        let public_key = signing_key.verifying_key();
        
        let from = hex::encode(public_key.to_bytes());
        let mut tx = Transaction::new(
            from,
            "recipient".to_string(),
            100,
            None,
        );
        tx.sign(&signing_key)?;
        
        let block = Block::new(
            1,
            "0".repeat(64),
            vec![tx],
            &signing_key,
        );
        
        // hash_data returns raw bytes, hash returns hex string
        let hash_data_bytes = block.hash_data();
        let hash_string = block.hash();
        
        // Note: hash_data uses big-endian, hash uses little-endian
        // They serve different purposes - hash_data for signing, hash for identification
        assert_eq!(hash_data_bytes.len(), 32); // blake3 raw output
        assert_eq!(hash_string.len(), 64); // hex string
        
        // Verify hash_data is consistent
        let hash_data_bytes2 = block.hash_data();
        assert_eq!(hash_data_bytes, hash_data_bytes2);
        
        Ok(())
    }

    #[test]
    fn test_merkle_root_with_odd_transactions() -> Result<(), Box<dyn std::error::Error>> {
        let signing_key = crypto_utils::default_signing_key();
        let public_key = signing_key.verifying_key();
        
        let from = hex::encode(public_key.to_bytes());
        
        // Create 3 transactions (odd number)
        let mut tx1 = Transaction::new(from.clone(), "to1".to_string(), 100, None);
        tx1.sign(&signing_key)?;
        
        let mut tx2 = Transaction::new(from.clone(), "to2".to_string(), 200, None);
        tx2.sign(&signing_key)?;
        
        let mut tx3 = Transaction::new(from, "to3".to_string(), 300, None);
        tx3.sign(&signing_key)?;
        
        let root = Block::calculate_merkle_root(&[tx1, tx2, tx3]);
        
        // Should handle odd number of transactions by duplicating the last one
        assert_eq!(root.len(), 64);
        assert!(root.chars().all(|c| c.is_ascii_hexdigit()));
        
        Ok(())
    }

    #[test]
    fn test_block_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let signing_key = crypto_utils::default_signing_key();
        let public_key = signing_key.verifying_key();
        
        let from = hex::encode(public_key.to_bytes());
        let mut tx = Transaction::new(
            from,
            "recipient".to_string(),
            100,
            None,
        );
        tx.sign(&signing_key)?;
        
        let block = Block::new(
            1,
            "0".repeat(64),
            vec![tx],
            &signing_key,
        );
        
        // Test JSON serialization
        let json = serde_json::to_string(&block)?;
        let deserialized_block: Block = serde_json::from_str(&json)?;
        
        assert_eq!(block.header.height, deserialized_block.header.height);
        assert_eq!(block.header.previous_hash, deserialized_block.header.previous_hash);
        assert_eq!(block.transactions.len(), deserialized_block.transactions.len());
        assert_eq!(block.signature, deserialized_block.signature);
        
        Ok(())
    }
}