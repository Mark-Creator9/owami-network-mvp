use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, VerifyingKey};
use blake3;
use hex;
use crate::crypto_utils;
use anyhow::Result;
use crate::audit_log;

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
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Transaction {
            from,
            to,
            amount,
            timestamp,
            signature: Vec::new(),
            data,
        }
    }

    pub fn sign(&mut self, signing_key: &SigningKey) -> Result<()> {
        let signature = crypto_utils::sign_message(signing_key, &self.hash_data());
        self.signature = crypto_utils::signature_to_bytes(&signature);
        
        audit_log::log_transaction_event(
            "Transaction signed".to_string(),
            format!("Transaction from {} to {} signed", self.from, self.to),
            "success".to_string(),
            Some(self.hash()),
            None,
        )?;
        
        Ok(())
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
            _ => {
                let _ = audit_log::log_security_event(
                    "Transaction verification failed".to_string(),
                    format!("Invalid from address format: {}", self.from),
                    "failure".to_string(),
                    Some(self.hash()),
                );
                return false;
            },
        };
        
        let public_key = match VerifyingKey::from_bytes(&public_key_bytes) {
            Ok(pk) => pk,
            Err(_) => {
                let _ = audit_log::log_security_event(
                    "Transaction verification failed".to_string(),
                    format!("Invalid public key from address: {}", self.from),
                    "failure".to_string(),
                    Some(self.hash()),
                );
                return false;
            },
        };
        
        let signature = match crypto_utils::signature_from_bytes(&self.signature) {
            Ok(sig) => sig,
            Err(_) => {
                let _ = audit_log::log_security_event(
                    "Transaction verification failed".to_string(),
                    "Invalid signature format".to_string(),
                    "failure".to_string(),
                    Some(self.hash()),
                );
                return false;
            },
        };
        
        let result = crypto_utils::verify_signature(&public_key, &self.hash_data(), &signature);
        
        if result {
            let _ = audit_log::log_security_event(
                "Transaction verified".to_string(),
                "Transaction signature verification successful".to_string(),
                "success".to_string(),
                Some(self.hash()),
            );
        } else {
            let _ = audit_log::log_security_event(
                "Transaction verification failed".to_string(),
                "Signature verification failed".to_string(),
                "failure".to_string(),
                Some(self.hash()),
            );
        }
        
        result
    }

    pub fn hash(&self) -> String {
        hex::encode(self.hash_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() -> Result<()> {
        let (signing_key, public_key) = crypto_utils::generate_keypair();
        
        let from = hex::encode(public_key.to_bytes());
        let to = "recipient_address".to_string();
        
        let mut tx = Transaction::new(
            from.clone(),
            to.clone(),
            100,
            None,
        );
        
        tx.sign(&signing_key)?;
        
        assert_eq!(tx.from, from);
        assert_eq!(tx.to, to);
        assert_eq!(tx.amount, 100);
        assert!(tx.verify());
        Ok(())
    }
}