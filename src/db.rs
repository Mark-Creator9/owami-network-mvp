use sqlx::{Pool, postgres::PgPoolOptions, Row};
use crate::config::AppConfig;

pub async fn create_pool(config: &AppConfig) -> Result<Pool<sqlx::Postgres>, Box<dyn std::error::Error>> {
    let database_url = config.database_url();
    
    let pool = PgPoolOptions::new()
        .max_connections(config.database.pool_size)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

// Database repository for blockchain operations
#[derive(Debug)]
pub struct BlockchainRepository {
    pool: Pool<sqlx::Postgres>,
}

impl BlockchainRepository {
    pub fn new(pool: Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_balance(&self, address: &str) -> Result<u64, sqlx::Error> {
        let row = sqlx::query("SELECT balance FROM balances WHERE address = $1")
            .bind(address)
            .fetch_optional(&self.pool)
            .await?;

        let balance = if let Some(row) = row {
            row.try_get::<i64, _>("balance")? as u64
        } else {
            0
        };

        Ok(balance)
    }

    pub async fn update_balance(&self, address: &str, balance: u64) -> Result<(), sqlx::Error> {
        // Use proper TIMESTAMP value for Postgres
        let timestamp = chrono::Utc::now().naive_utc();
        sqlx::query(
            "INSERT INTO balances (address, balance, updated_at) VALUES ($1, $2, $3)
             ON CONFLICT (address) DO UPDATE SET balance = $2, updated_at = $3"
        )
        .bind(address)
        .bind(balance as i64)
        .bind(timestamp)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn add_transaction(&self, transaction_data: &[u8]) -> Result<i64, sqlx::Error> {
        // Use proper TIMESTAMP value for Postgres
        let timestamp = chrono::Utc::now().naive_utc();
        let row = sqlx::query(
            "INSERT INTO pending_transactions (transaction_data, created_at) VALUES ($1, $2) RETURNING id"
        )
        .bind(transaction_data)
        .bind(timestamp)
        .fetch_one(&self.pool)
        .await?;
        let id = row.try_get::<i64, _>("id")?;

        Ok(id)
    }

    pub async fn get_pending_transactions(&self, limit: u32) -> Result<Vec<Vec<u8>>, sqlx::Error> {
        let rows = sqlx::query("SELECT transaction_data FROM pending_transactions ORDER BY created_at ASC LIMIT $1")
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        let transactions = rows
            .iter()
            .map(|row| row.try_get::<Vec<u8>, _>("transaction_data"))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(transactions)
    }

    pub async fn clear_pending_transactions(&self, ids: &[i64]) -> Result<(), sqlx::Error> {
        if ids.is_empty() {
            return Ok(());
        }
        
        sqlx::query("DELETE FROM pending_transactions WHERE id = ANY($1)")
            .bind(ids)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_latest_block(&self) -> Result<Option<Vec<u8>>, sqlx::Error> {
        let row = sqlx::query("SELECT transactions FROM blocks ORDER BY height DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await?;
        
        let block = if let Some(row) = row {
            Some(row.try_get::<Vec<u8>, _>("transactions")?)
        } else {
            None
        };

        Ok(block)
    }

    pub async fn get_block_count(&self) -> Result<u64, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM blocks")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.try_get::<i64, _>("count")? as u64)
    }

    pub async fn get_pending_transaction_count(&self) -> Result<u64, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM pending_transactions")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.try_get::<i64, _>("count")? as u64)
    }

    pub async fn get_height(&self) -> Result<u64, sqlx::Error> {
        let row = sqlx::query("SELECT COALESCE(MAX(height), 0) as height FROM blocks")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.try_get::<i64, _>("height")? as u64)
    }

    pub async fn get_blocks(&self, start: u64, limit: u64) -> Result<Vec<Vec<u8>>, sqlx::Error> {
        let rows = sqlx::query("SELECT transactions FROM blocks WHERE height >= $1 ORDER BY height ASC LIMIT $2")
            .bind(start as i64)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        let blocks = rows
            .iter()
            .map(|row| row.try_get::<Vec<u8>, _>("transactions"))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(blocks)
    }

    pub async fn add_block(&self, block_data: &[u8]) -> Result<i32, sqlx::Error> {
        // Deserialize the block to get the required fields
        use crate::block::Block;
        use bincode::deserialize;
        
        let block: Block = deserialize(block_data)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        
        let height = block.header.height as i64;
        let previous_hash = block.header.previous_hash.clone(); // Clone to avoid move
        let hash = block.hash();
        let timestamp = block.header.timestamp as i64;
        let validator = "validator".to_string(); // Placeholder, should get from block if available
        
        let row = sqlx::query(
            "INSERT INTO blocks (height, previous_hash, hash, timestamp, validator, transactions)
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
        )
        .bind(height)
        .bind(previous_hash)
        .bind(hash)
        .bind(timestamp)
        .bind(validator)
        .bind(block_data)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.try_get::<i32, _>("id")?)
    }
}
