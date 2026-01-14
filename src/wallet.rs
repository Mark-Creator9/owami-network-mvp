use crate::audit_log;
use crate::key_management::KeyManager;
use anyhow::Result;
use ed25519_dalek::{Signature, Signer, Verifier};
use hex;
use tempfile;

pub struct Wallet {
    key_manager: KeyManager,
}

impl Wallet {
    pub fn new() -> Result<Self> {
        let mut key_manager = KeyManager::new(None)?;
        key_manager.load_or_generate_key()?;

        audit_log::log_key_management_event(
            "Wallet created".to_string(),
            "New wallet created with generated key".to_string(),
            "success".to_string(),
            None,
        )?;

        Ok(Wallet { key_manager })
    }

    pub fn from_private_key(private_key: &str) -> Result<Self> {
        let bytes = hex::decode(private_key).map_err(|e| {
            let _ = audit_log::log_security_event(
                "Invalid private key".to_string(),
                format!("Failed to decode private key: {}", e),
                "failure".to_string(),
                None,
            );
            anyhow::anyhow!("Invalid private key: {}", e)
        })?;

        if bytes.len() != 32 {
            let _ = audit_log::log_security_event(
                "Invalid private key length".to_string(),
                format!("Private key has invalid length: {}", bytes.len()),
                "failure".to_string(),
                None,
            );
            return Err(anyhow::anyhow!("Invalid private key length"));
        }

        let temp_dir = tempfile::tempdir()?;
        let temp_key_path = temp_dir.path().join("temp.key");
        std::fs::write(&temp_key_path, &bytes)?;

        let mut key_manager = KeyManager::new(Some(temp_key_path.to_str().unwrap().to_string()))?;
        key_manager.load_or_generate_key()?;

        audit_log::log_key_management_event(
            "Wallet created from private key".to_string(),
            "Wallet created from provided private key".to_string(),
            "success".to_string(),
            None,
        )?;

        Ok(Wallet { key_manager })
    }

    pub fn address(&mut self) -> Result<String> {
        let verifying_key = self.key_manager.get_verifying_key()?;
        Ok(hex::encode(verifying_key.to_bytes()))
    }

    pub fn private_key(&mut self) -> Result<String> {
        let signing_key = self.key_manager.get_signing_key()?;
        let private_key = hex::encode(signing_key.to_bytes());

        audit_log::log_key_management_event(
            "Private key accessed".to_string(),
            "Private key retrieved from wallet".to_string(),
            "success".to_string(),
            None,
        )?;

        Ok(private_key)
    }

    pub fn sign(&mut self, message: &[u8]) -> Result<Vec<u8>> {
        let signing_key = self.key_manager.get_signing_key()?;
        let signature = signing_key.sign(message);

        audit_log::log_transaction_event(
            "Message signed".to_string(),
            format!("Signed message of length: {}", message.len()),
            "success".to_string(),
            None,
            None,
        )?;

        Ok(signature.to_bytes().to_vec())
    }

    pub fn verify(&mut self, message: &[u8], signature: &[u8]) -> Result<bool> {
        let verifying_key = self.key_manager.get_verifying_key()?;
        let signature = Signature::from_slice(signature).map_err(|e| {
            let _ = audit_log::log_security_event(
                "Signature verification failed".to_string(),
                format!("Invalid signature format: {}", e),
                "failure".to_string(),
                None,
            );
            anyhow::anyhow!("Invalid signature: {}", e)
        })?;

        let result = verifying_key
            .verify(message, &signature)
            .map_err(|e| {
                let _ = audit_log::log_security_event(
                    "Signature verification failed".to_string(),
                    format!("Signature verification error: {}", e),
                    "failure".to_string(),
                    None,
                );
                anyhow::anyhow!("Verification failed: {}", e)
            })
            .map(|_| true);

        if result.is_ok() {
            audit_log::log_security_event(
                "Signature verified".to_string(),
                "Signature verification successful".to_string(),
                "success".to_string(),
                None,
            )?;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use tempfile::tempdir; // Unused import
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    use rand::RngCore;

    #[test]
    fn test_wallet_creation() -> Result<()> {
        let mut wallet = Wallet::new()?;
        assert!(!wallet.address()?.is_empty());
        assert!(!wallet.private_key()?.is_empty());
        Ok(())
    }

    #[test]
    fn test_wallet_from_private_key() -> Result<()> {
        let mut rng = OsRng;
        let mut secret_key_bytes = [0u8; 32];
        rng.fill_bytes(&mut secret_key_bytes);
        let signing_key = SigningKey::from_bytes(&secret_key_bytes);
        let private_key = hex::encode(signing_key.to_bytes());

        let mut wallet = Wallet::from_private_key(&private_key)?;
        assert_eq!(
            wallet.address()?,
            hex::encode(signing_key.verifying_key().to_bytes())
        );
        Ok(())
    }

    #[test]
    fn test_invalid_private_key() {
        assert!(Wallet::from_private_key("invalid_key").is_err());
    }

    #[test]
    fn test_sign_verify_message() -> Result<()> {
        let mut wallet = Wallet::new()?;
        let message = b"test message";
        let signature = wallet.sign(message)?;

        assert!(wallet.verify(message, &signature)?);
        Ok(())
    }
}
