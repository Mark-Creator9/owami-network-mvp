use crate::block::Block;
use crate::transaction::Transaction;
use crate::consensus::dpos::{DposConsensus, Validator, SerializableVerifyingKey};
use crate::config::AppConfig as Config;
use crate::crypto_utils;
use std::collections::HashMap;
use chrono::Utc;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub consensus: DposConsensus,
    pub validator_set: HashMap<String, Validator>,
}

impl Blockchain {
    pub fn new(config: &Config) -> Self {
        // Create initial validator set
        let initial_validators = vec![
            Validator {
                address: SerializableVerifyingKey(crypto_utils::default_verifying_key()),
                stake: 100000,
                uptime: 1.0,
                missed_blocks: 0,
                last_active: Utc::now().timestamp(),
            }
        ];
        
        let consensus = DposConsensus::new(
            &config.consensus.dpos,
            initial_validators
        );

        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            pending_transactions: Vec::new(),
            consensus,
            validator_set: HashMap::new(),
        };

        // Create genesis block
        let genesis_block = Block::new(
            0,
            "0".repeat(64),
            Vec::new(),
            &crypto_utils::default_signing_key(),
        );
        
        blockchain.blocks.push(genesis_block);
        
        // Update consensus state
        blockchain.consensus.last_block_time = Utc::now().timestamp();
        
        blockchain
    }
    
    pub fn add_block(&mut self, block: Block) -> Result<(), String> {
        // Verify the block
        if !self.verify_block(&block) {
            return Err("Invalid block".to_string());
        }
        
        // Add to chain
        self.blocks.push(block);
        
        // Update consensus
        self.consensus.last_block_time = Utc::now().timestamp();
        self.consensus.update_validator_set();
        
        Ok(())
    }
    
    pub fn verify_block(&self, block: &Block) -> bool {
        // Check if block connects to the chain
        if let Some(last_block) = self.blocks.last() {
            if block.header.previous_hash != last_block.hash() {
                return false;
            }
        }
        
        // Verify block signature
        // This would typically involve checking against the expected validator
        true
    }
    
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        // Verify transaction
        if !transaction.verify() {
            return Err("Invalid transaction".to_string());
        }
        
        self.pending_transactions.push(transaction);
        Ok(())
    }
    
    pub fn mine_block(&mut self, signing_key: &ed25519_dalek::SigningKey) -> Result<Block, String> {
        // Select block producer
        let _producer = self.consensus.elect_block_producer();
        
        // Create new block with pending transactions
        let new_block = Block::new(
            self.blocks.len() as u64,
            self.blocks.last().unwrap().hash(),
            self.pending_transactions.clone(),
            signing_key,
        );
        
        // Clear pending transactions
        self.pending_transactions.clear();
        
        // Add block to chain
        self.add_block(new_block.clone())?;
        
        Ok(new_block)
    }
    
    pub fn get_block_height(&self) -> u64 {
        self.blocks.len() as u64 - 1
    }
    
    pub fn get_latest_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }
    
    pub fn get_block_by_height(&self, height: u64) -> Option<&Block> {
        self.blocks.get(height as usize)
    }
    
    // Add the missing methods for token functionality
    pub fn get_balance(&self, _address: &str) -> Result<u64, String> {
        // For now, we'll just return a default balance
        // In a real implementation, we would calculate the actual balance
        Ok(1000)
    }
    
    pub fn mint(&mut self, _address: String, _amount: u64) -> Result<(), String> {
        // For now, we'll just return Ok
        // In a real implementation, we would update the balance
        Ok(())
    }
}