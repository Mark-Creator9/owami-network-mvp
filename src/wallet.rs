use ed25519_dalek::{Signer, SigningKey, Verifier, Signature};
use rand::rngs::OsRng;
use rand::RngCore;
use hex;

pub struct Wallet {
    signing_key: SigningKey,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let mut secret_key_bytes = [0u8; 32];
        rng.fill_bytes(&mut secret_key_bytes);
        let signing_key = SigningKey::from_bytes(&secret_key_bytes);
        Wallet { signing_key }
    }

    pub fn from_private_key(private_key: &str) -> Result<Self, String> {
        let bytes = hex::decode(private_key).map_err(|_| "Invalid private key".to_string())?;
        if bytes.len() != 32 {
            return Err("Invalid private key length".to_string());
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        let signing_key = SigningKey::from_bytes(&array);
        Ok(Wallet { signing_key })
    }

    pub fn address(&self) -> String {
        hex::encode(self.signing_key.verifying_key().to_bytes())
    }

    pub fn private_key(&self) -> Option<String> {
        Some(hex::encode(self.signing_key.to_bytes()))
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        let signature = self.signing_key.sign(message);
        Ok(signature.to_bytes().to_vec())
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, String> {
        let verifying_key = self.signing_key.verifying_key();
        let signature = Signature::from_slice(signature)
            .map_err(|e| e.to_string())?;
        verifying_key.verify(message, &signature)
            .map_err(|e| e.to_string())
            .map(|_| true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new();
        assert!(!wallet.address().is_empty());
        assert!(wallet.private_key().is_some());
    }

    #[test]
    fn test_wallet_from_private_key() {
        let mut rng = OsRng;
        let mut secret_key_bytes = [0u8; 32];
        rng.fill_bytes(&mut secret_key_bytes);
        let signing_key = SigningKey::from_bytes(&secret_key_bytes);
        let private_key = hex::encode(signing_key.to_bytes());
        
        let wallet = Wallet::from_private_key(&private_key).unwrap();
        assert_eq!(wallet.address(), hex::encode(signing_key.verifying_key().to_bytes()));
    }

    #[test]
    fn test_invalid_private_key() {
        assert!(Wallet::from_private_key("invalid_key").is_err());
    }

    #[test]
    fn test_sign_verify_message() {
        let wallet = Wallet::new();
        let message = b"test message";
        let signature = wallet.sign(message).unwrap();
        
        assert!(wallet.verify(message, &signature).unwrap());
    }
}