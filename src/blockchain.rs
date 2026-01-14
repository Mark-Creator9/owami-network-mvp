use crate::block::Block;
use crate::config::AppConfig as Config;
use crate::consensus::dpos::{DposConsensus, SerializableVerifyingKey, Validator};
use crate::crypto_utils;
use crate::transaction::Transaction;
use chrono::Utc;
use std::collections::HashMap;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub balances: HashMap<String, u64>, // Track token balances
    pub consensus: DposConsensus,
    pub validator_set: HashMap<String, Validator>,
}

impl Blockchain {
    pub fn new(config: &Config) -> Self {
        // Create initial validator set
        let initial_validators = vec![Validator {
            address: SerializableVerifyingKey(crypto_utils::default_verifying_key()),
            stake: 100000,
            uptime: 1.0,
            missed_blocks: 0,
            last_active: Utc::now().timestamp(),
        }];

        let consensus = DposConsensus::new(&config.consensus.dpos, initial_validators);

        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            pending_transactions: Vec::new(),
            balances: HashMap::new(), // Initialize balance tracking
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
        // Verify block
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
        // Check if block connects to chain
        if let Some(last_block) = self.blocks.last() {
            if block.header.previous_hash != last_block.hash() {
                return false;
            }
        }

        // Verify block signature
        // This would typically involve checking against expected validator
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

    // Token functionality
    pub fn get_balance(&self, address: &str) -> u64 {
        // Get balance from tracking map
        *self.balances.get(address).unwrap_or(&0)
    }

    pub fn mint(&mut self, address: String, amount: u64) -> Result<(), String> {
        let current_balance = *self.balances.get(&address).unwrap_or(&0);
        let _ = self
            .balances
            .entry(address)
            .or_insert(current_balance + amount);
        Ok(())
    }
}
