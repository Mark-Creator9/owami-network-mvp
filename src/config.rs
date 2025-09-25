use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub pool_size: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonitoringConfig {
    pub health_check_interval: u64,
    pub metrics_port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityConfig {
    pub cors_origins: Vec<String>,
    pub rate_limiting: RateLimitingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitingConfig {
    pub requests: u32,
    pub per_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub monitoring: MonitoringConfig,
    pub security: SecurityConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = std::env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "config/production.toml".to_string());
        
        let config_content = fs::read_to_string(&config_path)?;
        let config: AppConfig = toml::from_str(&config_content)?;
        
        Ok(config)
    }
    
    pub fn database_url(&self) -> String {
        std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:owami_testnet.db".to_string())
    }
}