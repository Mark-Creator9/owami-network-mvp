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
    pub data_dir: String,
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
pub struct ConsensusConfig {
    pub consensus_type: String, // "poa", "dpos", etc.
    pub dpos: DposConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DposConfig {
    pub validator_count: u32,
    pub block_interval: u64,    // in seconds
    pub stake_threshold: u64,
    pub slashing_penalty: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub monitoring: MonitoringConfig,
    pub security: SecurityConfig,
    pub consensus: ConsensusConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = std::env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "config/production.toml".to_string());

        let config_content = fs::read_to_string(&config_path)?;
        let mut config: AppConfig = toml::from_str(&config_content)?;

        if let Ok(port_str) = std::env::var("PORT") {
            if let Ok(port) = port_str.parse::<u16>() {
                config.server.port = port;
            }
        }

        Ok(config)
    }
}