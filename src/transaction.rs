use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, VerifyingKey};
use blake3;
use hex;
use crate::crypto_utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: u64,
    pub signature: Vec<u8>,
    pub data: Option<String>,
}

impl Transaction {
    pub fn new(
        from: String,
        to: String,
        amount: u64,
        data: Option<String>,
        signing_key: &SigningKey,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut tx = Transaction {
            from,
            to,
            amount,
            timestamp,
            signature: Vec::new(),
            data,
        };

        let signature = crypto_utils::sign_message(signing_key, &tx.hash_data());
        tx.signature = crypto_utils::signature_to_bytes(&signature);
        tx
    }

    fn hash_data(&self) -> Vec<u8> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.from.as_bytes());
        hasher.update(self.to.as_bytes());
        hasher.update(&self.amount.to_le_bytes());
        hasher.update(&self.timestamp.to_le_bytes());
        if let Some(data) = &self.data {
            hasher.update(data.as_bytes());
        }
        hasher.finalize().as_bytes().to_vec()
    }

    pub fn verify(&self) -> bool {
        // For now, we'll use a simplified verification
        // In a real implementation, we'd extract the public key from the signature
        // or require it to be passed separately
        
        // Try to decode the address as a public key
        let public_key_bytes = match hex::decode(&self.from) {
            Ok(bytes) if bytes.len() == 32 => {
                let mut key_bytes = [0u8; 32];
                key_bytes.copy_from_slice(&bytes);
                key_bytes
            },
            _ => return false,
        };
        
        let public_key = match VerifyingKey::from_bytes(&public_key_bytes) {
            Ok(pk) => pk,
            Err(_) => return false,
        };
        
        let signature = match crypto_utils::signature_from_bytes(&self.signature) {
            Ok(sig) => sig,
            Err(_) => return false,
        };
        
        crypto_utils::verify_signature(&public_key, &self.hash_data(), &signature)
    }

    pub fn hash(&self) -> String {
        hex::encode(self.hash_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[test]
    fn test_transaction_creation() {
        let signing_key = crypto_utils::generate_keypair();
        let public_key = signing_key.verifying_key();
        
        let from = hex::encode(public_key.to_bytes());
        let to = "recipient_address".to_string();
        
        let tx = Transaction::new(
            from.clone(),
            to.clone(),
            100,
            None,
            &signing_key,
        );
        
        assert_eq!(tx.from, from);
        assert_eq!(tx.to, to);
        assert_eq!(tx.amount, 100);
        assert!(tx.verify());
    }
}