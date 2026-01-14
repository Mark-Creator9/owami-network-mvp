use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, info, warn};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AuditEventType {
    KeyManagement,
    Transaction,
    BlockMining,
    Security,
    System,
    Authentication,
    Authorization,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLogEntry {
    pub timestamp: u64,
    pub event_type: AuditEventType,
    pub user_id: Option<String>,
    pub ip_address: Option<String>,
    pub action: String,
    pub details: String,
    pub status: String, // "success", "failure", "warning"
    pub resource: Option<String>,
}

pub struct AuditLogger {
    log_file: Mutex<File>,
}

impl AuditLogger {
    pub fn new(log_path: &str) -> Result<Self> {
        // Ensure directory exists
        if let Some(parent) = Path::new(log_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;

        Ok(AuditLogger {
            log_file: Mutex::new(file),
        })
    }

    pub fn log_event(&self, entry: AuditLogEntry) -> Result<()> {
        let json_entry = serde_json::to_string(&entry)?;

        let mut file = self.log_file.lock().unwrap();
        writeln!(file, "{}", json_entry)?;

        // Also log to tracing based on status
        match entry.status.as_str() {
            "success" => info!("AUDIT: {} - {}", entry.action, entry.details),
            "failure" => error!("AUDIT FAILURE: {} - {}", entry.action, entry.details),
            "warning" => warn!("AUDIT WARNING: {} - {}", entry.action, entry.details),
            _ => info!("AUDIT: {} - {}", entry.action, entry.details),
        }

        Ok(())
    }

    pub fn create_entry(
        event_type: AuditEventType,
        action: String,
        details: String,
        status: String,
        user_id: Option<String>,
        ip_address: Option<String>,
        resource: Option<String>,
    ) -> AuditLogEntry {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        AuditLogEntry {
            timestamp,
            event_type,
            user_id,
            ip_address,
            action,
            details,
            status,
            resource,
        }
    }
}

// Global audit logger instance
lazy_static! {
    static ref AUDIT_LOGGER: Mutex<Option<AuditLogger>> = Mutex::new(None);
}

pub fn initialize_audit_logger(log_path: Option<String>) -> Result<()> {
    let path = log_path.unwrap_or_else(|| {
        std::env::var("AUDIT_LOG_PATH").unwrap_or_else(|_| "./logs/audit.log".to_string())
    });

    let logger = AuditLogger::new(&path)?;
    *AUDIT_LOGGER.lock().unwrap() = Some(logger);

    info!("Audit logging initialized at: {}", path);
    Ok(())
}

pub fn log_audit_event(entry: AuditLogEntry) -> Result<()> {
    if let Some(logger) = AUDIT_LOGGER.lock().unwrap().as_ref() {
        logger.log_event(entry)
    } else {
        // Fallback to tracing if logger not initialized
        match entry.status.as_str() {
            "success" => info!("AUDIT (fallback): {} - {}", entry.action, entry.details),
            "failure" => error!(
                "AUDIT FAILURE (fallback): {} - {}",
                entry.action, entry.details
            ),
            "warning" => warn!(
                "AUDIT WARNING (fallback): {} - {}",
                entry.action, entry.details
            ),
            _ => info!("AUDIT (fallback): {} - {}", entry.action, entry.details),
        }
        Ok(())
    }
}

// Convenience functions for common audit events
pub fn log_key_management_event(
    action: String,
    details: String,
    status: String,
    user_id: Option<String>,
) -> Result<()> {
    let entry = AuditLogger::create_entry(
        AuditEventType::KeyManagement,
        action,
        details,
        status,
        user_id,
        None,
        None,
    );
    log_audit_event(entry)
}

pub fn log_transaction_event(
    action: String,
    details: String,
    status: String,
    user_id: Option<String>,
    resource: Option<String>,
) -> Result<()> {
    let entry = AuditLogger::create_entry(
        AuditEventType::Transaction,
        action,
        details,
        status,
        user_id,
        None,
        resource,
    );
    log_audit_event(entry)
}

pub fn log_security_event(
    action: String,
    details: String,
    status: String,
    user_id: Option<String>,
) -> Result<()> {
    let entry = AuditLogger::create_entry(
        AuditEventType::Security,
        action,
        details,
        status,
        user_id,
        None,
        None,
    );
    log_audit_event(entry)
}

pub fn log_system_event(action: String, details: String, status: String) -> Result<()> {
    let entry = AuditLogger::create_entry(
        AuditEventType::System,
        action,
        details,
        status,
        None,
        None,
        None,
    );
    log_audit_event(entry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_audit_logger_creation() -> Result<()> {
        let temp_dir = tempdir()?;
        let log_path = temp_dir
            .path()
            .join("test_audit.log")
            .to_str()
            .unwrap()
            .to_string();

        let _logger = AuditLogger::new(&log_path)?;
        assert!(Path::new(&log_path).exists());
        Ok(())
    }

    #[test]
    fn test_audit_log_entry() -> Result<()> {
        let entry = AuditLogger::create_entry(
            AuditEventType::KeyManagement,
            "Key generated".to_string(),
            "New signing key created".to_string(),
            "success".to_string(),
            Some("user123".to_string()),
            None,
            None,
        );

        assert_eq!(entry.action, "Key generated");
        assert_eq!(entry.status, "success");
        Ok(())
    }
}
