pub mod key_manager;
pub mod middleware;
pub mod routes;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use std::env;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize, // issued at
    pub exp: usize, // expiration
}

#[derive(Clone)]
pub struct JwtConfig {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey, 
    pub validation: Validation,
}

impl JwtConfig {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
            validation: Validation::new(Algorithm::HS256),
        }
    }
}

pub fn create_jwt_config() -> JwtConfig {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
        log::warn!("Using default JWT_SECRET - insecure for production!");
        "insecure_default_secret".to_string()
    });
    
    JwtConfig::new(secret.as_bytes())
}