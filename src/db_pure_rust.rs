// Pure Rust database implementation - alternative to RocksDB
// This avoids the libclang dependency issue

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, RwLock};

pub struct PureRustDatabase {
    data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    data_dir: String,
}

impl PureRustDatabase {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Create data directory if it doesn't exist
        fs::create_dir_all(path)?;

        let db = PureRustDatabase {
            data: Arc::new(RwLock::new(HashMap::new())),
            data_dir: path.to_string(),
        };

        // Load any existing data from files
        db.load_from_disk()?;

        Ok(db)
    }

    fn load_from_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut data = self.data.write().unwrap();

        if Path::new(&self.data_dir).exists() {
            for entry in fs::read_dir(&self.data_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    let mut file = File::open(&path)?;
                    let mut contents = Vec::new();
                    file.read_to_end(&mut contents)?;

                    if let Some(key) = path.file_name() {
                        let key_str = key.to_string_lossy().into_owned();
                        data.insert(key_str, contents);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let key_str = String::from_utf8_lossy(key).into_owned();

        // Update in-memory data
        {
            let mut data = self.data.write().unwrap();
            data.insert(key_str.clone(), value.to_vec());
        }

        // Write to disk
        let file_path = Path::new(&self.data_dir).join(&key_str);
        let mut file = File::create(file_path)?;
        file.write_all(value)?;

        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        let key_str = String::from_utf8_lossy(key).into_owned();
        let data = self.data.read().unwrap();

        Ok(data.get(&key_str).cloned())
    }

    pub fn delete(&self, key: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let key_str = String::from_utf8_lossy(key).into_owned();

        // Remove from memory
        {
            let mut data = self.data.write().unwrap();
            data.remove(&key_str);
        }

        // Remove from disk
        let file_path = Path::new(&self.data_dir).join(&key_str);
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }

        Ok(())
    }

    // Simple health check - just verify directory exists
    pub fn health_check() -> String {
        if Path::new("data/rocksdb").exists() {
            "connected".to_string()
        } else {
            "disconnected".to_string()
        }
    }
}
