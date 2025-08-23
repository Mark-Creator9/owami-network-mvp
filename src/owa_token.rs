use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub address: String,
    pub balance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub from: String,
    pub to: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveRequest {
    pub owner: String,
    pub spender: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub tx_hash: Option<String>,
}

#[derive(Debug)]
pub struct TokenState {
    balances: Arc<RwLock<HashMap<String, u128>>>,
    transactions: Arc<RwLock<Vec<Transaction>>>,
    total_supply: u128,
}

impl TokenState {
    pub fn new() -> Self {
        let mut balances = HashMap::new();
        
        // Initialize with some test balances
        balances.insert("0x742d35Cc6634C0532925a3b8D4e6D3b6e8d3e8A0".to_string(), 1_000_000_000_000_000_000_000_000); // 1M OWA
        balances.insert("0x1234567890123456789012345678901234567890".to_string(), 100_000_000_000_000_000_000); // 100 OWA
        balances.insert("0x0987654321098765432109876543210987654321".to_string(), 50_000_000_000_000_000_000); // 50 OWA
        
        Self {
            balances: Arc::new(RwLock::new(balances)),
            transactions: Arc::new(RwLock::new(Vec::new())),
            total_supply: 1_000_000_000_000_000_000_000_000, // 1M OWA with 18 decimals
        }
    }

    pub async fn get_token_info(&self) -> TokenInfo {
        TokenInfo {
            name: "OWami Token".to_string(),
            symbol: "OWA".to_string(),
            decimals: 18,
            total_supply: self.total_supply.to_string(),
            address: "0x742d35Cc6634C0532925a3b8D4e6D3b6e8d3e8A0".to_string(),
        }
    }

    pub async fn get_balance(&self, address: &str) -> Balance {
        let balances = self.balances.read().await;
        let balance = balances.get(address).copied().unwrap_or(0);
        
        Balance {
            address: address.to_string(),
            balance: balance.to_string(),
        }
    }

    pub async fn transfer(&self, from: &str, to: &str, amount_str: &str) -> Result<String, String> {
        let amount = amount_str.parse::<u128>().map_err(|_| "Invalid amount")?;
        
        if amount == 0 {
            return Err("Amount must be greater than 0".to_string());
        }

        let mut balances = self.balances.write().await;
        
        let from_balance = balances.get(from).copied().unwrap_or(0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        *balances.get_mut(from).unwrap() -= amount;
        *balances.entry(to.to_string()).or_insert(0) += amount;

        // Record transaction
        let tx_hash = format!("0x{:x}", rand::random::<u64>());
        let transaction = Transaction {
            tx_hash: tx_hash.clone(),
            from: from.to_string(),
            to: to.to_string(),
            amount: amount.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        let mut transactions = self.transactions.write().await;
        transactions.push(transaction);

        Ok(tx_hash)
    }

    pub async fn approve(&self, owner: &str, spender: &str, amount_str: &str) -> Result<String, String> {
        // For testnet, we'll just record this as a transaction
        let amount = amount_str.parse::<u128>().map_err(|_| "Invalid amount")?;
        
        let tx_hash = format!("0x{:x}", rand::random::<u64>());
        let transaction = Transaction {
            tx_hash: tx_hash.clone(),
            from: owner.to_string(),
            to: spender.to_string(),
            amount: amount.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        let mut transactions = self.transactions.write().await;
        transactions.push(transaction);

        Ok(tx_hash)
    }

    pub async fn mint(&self, to: &str, amount_str: &str) -> Result<String, String> {
        let amount = amount_str.parse::<u128>().map_err(|_| "Invalid amount")?;
        
        let mut balances = self.balances.write().await;
        *balances.entry(to.to_string()).or_insert(0) += amount;

        let tx_hash = format!("0x{:x}", rand::random::<u64>());
        let transaction = Transaction {
            tx_hash: tx_hash.clone(),
            from: "0x0000000000000000000000000000000000000000".to_string(),
            to: to.to_string(),
            amount: amount.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        let mut transactions = self.transactions.write().await;
        transactions.push(transaction);

        Ok(tx_hash)
    }

    pub async fn get_transactions(&self) -> Vec<Transaction> {
        let transactions = self.transactions.read().await;
        transactions.clone()
    }
}