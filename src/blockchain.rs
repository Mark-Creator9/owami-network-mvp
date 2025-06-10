use crate::block::Block;
use ed25519_dalek::SigningKey;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::mpsc;
use std::collections::VecDeque;
use rayon::prelude::*;

const BATCH_SIZE: usize = 100;
const MAX_PENDING_BATCHES: usize = 10;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pending_transactions: RwLock<VecDeque<Block>>,
    batch_processor: Option<mpsc::Sender<Vec<Block>>>,
}

impl Blockchain {
    pub fn new(validator_key: &SigningKey) -> Self {
        let genesis_block = Block::new(
            0,
            "0".repeat(64),
            Vec::new(),
            validator_key
        );
        Blockchain {
            blocks: vec![genesis_block],
            pending_transactions: RwLock::new(VecDeque::new()),
            batch_processor: None,
        }
    }

    pub async fn start_batch_processor(&mut self) {
        let (tx, mut rx) = mpsc::channel(MAX_PENDING_BATCHES);
        self.batch_processor = Some(tx);

        let blockchain = Arc::new(Mutex::new(self.clone()));
        
        tokio::spawn(async move {
            while let Some(batch) = rx.recv().await {
                let valid_blocks: Vec<Block> = batch
                    .into_par_iter()
                    .filter(|block| {
                        if let Ok(chain) = blockchain.lock() {
                            if let Some(latest) = chain.get_latest_block() {
                                block.header.height == latest.header.height + 1 &&
                                block.header.previous_hash == latest.hash()
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                    .collect();

                if let Ok(mut chain) = blockchain.lock() {
                    for block in valid_blocks {
                        chain.blocks.push(block);
                    }
                }
            }
        });
    }

    pub async fn add_block(&mut self, new_block: Block) -> Result<(), String> {
        if let Some(batch_sender) = &self.batch_processor {
            let mut pending = self.pending_transactions.write().unwrap();
            pending.push_back(new_block);

            if pending.len() >= BATCH_SIZE {
                let batch: Vec<Block> = pending.drain(..BATCH_SIZE).collect();
                batch_sender.send(batch).await.map_err(|e| e.to_string())?;
            }
            Ok(())
        } else {
            // Fallback to synchronous processing
            let latest_block = self.get_latest_block().ok_or("Blockchain is empty")?;
            
            if new_block.header.height != latest_block.header.height + 1 {
                return Err(format!(
                    "Invalid block height: expected {}, got {}",
                    latest_block.header.height + 1,
                    new_block.header.height
                ));
            }
            if new_block.header.previous_hash != latest_block.hash() {
                return Err("Invalid previous block hash".to_string());
            }

            self.blocks.push(new_block);
            Ok(())
        }
    }

    pub fn get_latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn clone(&self) -> Self {
        Blockchain {
            blocks: self.blocks.clone(),
            pending_transactions: RwLock::new(VecDeque::new()),
            batch_processor: None,
        }
    }
}

pub fn create_shared_blockchain(validator_key: &SigningKey) -> Arc<Mutex<Blockchain>> {
    let mut blockchain = Blockchain::new(validator_key);
    
    // Start the batch processor in a tokio runtime
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        blockchain.start_batch_processor().await;
    });

    Arc::new(Mutex::new(blockchain))
}
// (Please confirm you want me to show full file content)