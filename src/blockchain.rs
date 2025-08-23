use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, VerifyingKey};
use std::collections::HashMap;
use crate::block::Block;
use crate::transaction::Transaction;
use crate::crypto_utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub balances: HashMap<String, u64>,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(validator_key: &SigningKey) -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            balances: HashMap::new(),
            pending_transactions: Vec::new(),
        };
        
        // Create genesis block
        let genesis_block = Block::new(
            0,
            "0".repeat(64),
            Vec::new(),
            validator_key,
        );
        
        blockchain.blocks.push(genesis_block);
        blockchain
    }

    pub fn get_latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub async fn add_block(&mut self, block: Block) -> Result<(), String> {
        // Validate block
        let latest_block = self.get_latest_block()
            .ok_or("No latest block found")?;
        
        if block.header.height != latest_block.header.height + 1 {
            return Err("Invalid block height".to_string());
        }
        
        if block.header.previous_hash != latest_block.hash() {
            return Err("Invalid previous hash".to_string());
        }
        
        // Validate transactions
        for tx in &block.transactions {
            if !tx.verify() {
                return Err("Invalid transaction signature".to_string());
            }
            
            // Check balance
            let from_balance = self.balances.get(&tx.from).copied().unwrap_or(0);
            if from_balance < tx.amount {
                return Err("Insufficient balance".to_string());
            }
        }
        
        // Apply transactions
        for tx in &block.transactions {
            *self.balances.entry(tx.from.clone()).or_insert(0) -= tx.amount;
            *self.balances.entry(tx.to.clone()).or_insert(0) += tx.amount;
        }
        
        self.blocks.push(block);
        Ok(())
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        if !transaction.verify() {
            return Err("Invalid transaction signature".to_string());
        }
        
        let from_balance = self.balances.get(&transaction.from).copied().unwrap_or(0);
        if from_balance < transaction.amount {
            return Err("Insufficient balance".to_string());
        }
        
        self.pending_transactions.push(transaction);
        Ok(())
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.balances.get(address).copied().unwrap_or(0)
    }

    pub fn mint(&mut self, address: String, amount: u64) {
        *self.balances.entry(address).or_insert(0) += amount;
    }

    pub fn get_height(&self) -> u64 {
        self.get_latest_block()
            .map(|b| b.header.height)
            .unwrap_or(0)
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];
            
            if current.header.previous_hash != previous.hash() {
                return false;
            }
            
            if !current.verify_signature(&self.get_validator_key()) {
                return false;
            }
        }
        
        true
    }

    fn get_validator_key(&self) -> VerifyingKey {
        let signing_key = crypto_utils::default_signing_key();
        signing_key.verifying_key()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use crate::transaction::Transaction;

    #[tokio::test]
    async fn test_blockchain_creation() {
        let validator_key = crypto_utils::default_signing_key();
        let blockchain = Blockchain::new(&validator_key);
        
        assert_eq!(blockchain.blocks.len(), 1);
        assert_eq!(blockchain.get_height(), 0);
    }

    #[tokio::test]
    async fn test_add_block() {
        let validator_key = crypto_utils::default_signing_key();
        let mut blockchain = Blockchain::new(&validator_key);
        
        // Mint some tokens
        let public_key = validator_key.verifying_key();
        let address = hex::encode(public_key.to_bytes());
        blockchain.mint(address.clone(), 1000);
        
        // Create transaction
        let tx = Transaction::new(
            address.clone(),
            "recipient".to_string(),
            100,
            None,
            &validator_key,
        );
        
        // Create and add block
        let latest_block = blockchain.get_latest_block().unwrap();
        let block = Block::new(
            latest_block.header.height + 1,
            latest_block.hash(),
            vec![tx],
            &validator_key,
        );
        
        assert!(blockchain.add_block(block).await.is_ok());
        assert_eq!(blockchain.get_height(), 1);
        assert_eq!(blockchain.get_balance(&address), 900);
        assert_eq!(blockchain.get_balance("recipient"), 100);
    }

    #[test]
    fn test_mint_and_balance() {
        let validator_key = crypto_utils::default_signing_key();
        let mut blockchain = Blockchain::new(&validator_key);
        
        blockchain.mint("test_address".to_string(), 500);
        assert_eq!(blockchain.get_balance("test_address"), 500);
    }
}