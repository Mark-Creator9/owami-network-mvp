use rocksdb::{DB, Options};
use std::sync::Arc;

pub struct Database {
    db: Arc<DB>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = Arc::new(DB::open(&opts, path)?);
        Ok(Database { db })
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.db.put(key, value)?;
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        let result = self.db.get(key)?;
        Ok(result)
    }

    pub fn delete(&self, key: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.db.delete(key)?;
        Ok(())
    }
}