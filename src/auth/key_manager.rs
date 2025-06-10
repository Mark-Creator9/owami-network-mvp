use serde::{Deserialize, Serialize};
use rand::{distributions::Alphanumeric, Rng};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub email: String,
    pub permissions: Vec<String>,
    pub expires_in: Option<i32>
}

#[derive(Debug, Clone)]
pub struct ApiKeyManager {}

impl ApiKeyManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_key(&self, email: &str) -> ApiKey {
        let key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        ApiKey {
            key: format!("owa_{}", key),
            email: email.to_string(),
            permissions: vec!["read".to_string(), "write".to_string()],
            expires_in: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub secret: String
}