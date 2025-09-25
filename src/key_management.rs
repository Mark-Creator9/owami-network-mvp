use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::env;
use std::fs;
use std::path::Path;
use anyhow::{Result, anyhow};
use tracing::info;
use crate::audit_log;

/// Secure key management system for production use
pub struct KeyManager {
    signing_key: Option<SigningKey>,
    key_path: String,
}

impl KeyManager {
    /// Create a new KeyManager instance
    pub fn new(key_path: Option<String>) -> Result<Self> {
        let path = key_path.unwrap_or_else(|| {
            env::var("SIGNING_KEY_PATH").unwrap_or_else(|_| "./keys/validator.key".to_string())
        });
        
        // Ensure directory exists
        if let Some(parent) = Path::new(&path).parent() {
            fs::create_dir_all(parent)?;
        }
        
        Ok(KeyManager {
            signing_key: None,
            key_path: path,
        })
    }

    /// Load or generate signing key securely
    pub fn load_or_generate_key(&mut self) -> Result<SigningKey> {
        if let Some(key) = &self.signing_key {
            return Ok(key.clone());
        }

        // Try to load from file
        if Path::new(&self.key_path).exists() {
            info!("Loading signing key from: {}", self.key_path);
            let key_bytes = fs::read(&self.key_path)?;
            
            if key_bytes.len() != 32 {
                audit_log::log_key_management_event(
                    "Key load failed".to_string(),
                    format!("Invalid key length in file: {}", self.key_path),
                    "failure".to_string(),
                    None,
                )?;
                return Err(anyhow!("Invalid key length in file"));
            }
            
            let mut key_array = [0u8; 32];
            key_array.copy_from_slice(&key_bytes);
            let signing_key = SigningKey::from_bytes(&key_array);
            
            self.signing_key = Some(signing_key.clone());
            
            audit_log::log_key_management_event(
                "Key loaded".to_string(),
                format!("Signing key loaded from: {}", self.key_path),
                "success".to_string(),
                None,
            )?;
            
            return Ok(signing_key);
        }

        // Generate new key
        info!("Generating new signing key");
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        
        // Save to file securely
        self.save_key(&signing_key)?;
        
        self.signing_key = Some(signing_key.clone());
        
        audit_log::log_key_management_event(
            "Key generated".to_string(),
            format!("New signing key generated and saved to: {}", self.key_path),
            "success".to_string(),
            None,
        )?;
        
        Ok(signing_key)
    }

    /// Save key to secure storage
    fn save_key(&self, signing_key: &SigningKey) -> Result<()> {
        let key_bytes = signing_key.to_bytes();
        fs::write(&self.key_path, key_bytes)?;
        
        // Set secure permissions (Unix-like systems)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&self.key_path)?.permissions();
            perms.set_mode(0o600); // Read/write for owner only
            fs::set_permissions(&self.key_path, perms)?;
        }
        
        info!("Signing key saved to: {}", self.key_path);
        
        audit_log::log_key_management_event(
            "Key saved".to_string(),
            format!("Signing key saved to secure storage: {}", self.key_path),
            "success".to_string(),
            None,
        )?;
        
        Ok(())
    }

    /// Get the current signing key
    pub fn get_signing_key(&mut self) -> Result<SigningKey> {
        self.load_or_generate_key()
    }

    /// Get the verifying key
    pub fn get_verifying_key(&mut self) -> Result<VerifyingKey> {
        let signing_key = self.get_signing_key()?;
        Ok(signing_key.verifying_key())
    }

    /// Rotate the signing key (for security purposes)
    pub fn rotate_key(&mut self) -> Result<SigningKey> {
        info!("Rotating signing key");
        let mut rng = OsRng;
        let new_signing_key = SigningKey::generate(&mut rng);
        
        self.save_key(&new_signing_key)?;
        self.signing_key = Some(new_signing_key.clone());
        
        audit_log::log_key_management_event(
            "Key rotated".to_string(),
            "Signing key rotated for security purposes".to_string(),
            "success".to_string(),
            None,
        )?;
        
        Ok(new_signing_key)
    }

    /// Backup key to secure location
    pub fn backup_key(&self, backup_path: &str) -> Result<()> {
        let signing_key = self.signing_key
            .as_ref()
            .ok_or_else(|| anyhow!("No key loaded"))?;
        
        let key_bytes = signing_key.to_bytes();
        fs::write(backup_path, key_bytes)?;
        
        info!("Key backed up to: {}", backup_path);
        
        audit_log::log_key_management_event(
            "Key backup".to_string(),
            format!("Signing key backed up to: {}", backup_path),
            "success".to_string(),
            None,
        )?;
        
        Ok(())
    }
}

/// Initialize key manager from environment
pub fn initialize_key_manager() -> Result<KeyManager> {
    let key_path = env::var("SIGNING_KEY_PATH").ok();
    KeyManager::new(key_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_key_manager_creation() -> Result<()> {
        let temp_dir = tempdir()?;
        let key_path = temp_dir.path().join("test.key").to_str().unwrap().to_string();
        
        let mut manager = KeyManager::new(Some(key_path.clone()))?;
        let signing_key = manager.get_signing_key()?;
        
        assert!(Path::new(&key_path).exists());
        assert_eq!(signing_key.to_bytes().len(), 32);
        
        Ok(())
    }

    #[test]
    fn test_key_rotation() -> Result<()> {
        let temp_dir = tempdir()?;
        let key_path = temp_dir.path().join("rotate.key").to_str().unwrap().to_string();
        
        let mut manager = KeyManager::new(Some(key_path.clone()))?;
        let original_key = manager.get_signing_key()?;
        let rotated_key = manager.rotate_key()?;
        
        assert_ne!(original_key.to_bytes(), rotated_key.to_bytes());
        Ok(())
    }
}