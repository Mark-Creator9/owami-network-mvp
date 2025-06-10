use owami_network::{
    Blockchain,
    create_shared_blockchain,
    Block
};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use rand::RngCore;

#[test]
fn test_genesis_block_creation() {
    let mut rng = OsRng;
    let mut secret = [0u8; 32];
    rng.fill_bytes(&mut secret);
    let validator_key = SigningKey::from_bytes(&secret);
    let blockchain = Blockchain::new(&validator_key);
    
    assert_eq!(blockchain.blocks.len(), 1);
    assert_eq!(blockchain.blocks[0].header.height, 0);
    assert_eq!(blockchain.blocks[0].header.previous_hash, "0".repeat(64));
}

#[tokio::test]
async fn test_block_validation() {
    let mut rng = OsRng;
    let mut secret = [0u8; 32];
    rng.fill_bytes(&mut secret);
    let validator_key = SigningKey::from_bytes(&secret);
    let mut blockchain = Blockchain::new(&validator_key);
    
    // Create valid block
    let valid_block = Block::new(
        1,
        blockchain.blocks[0].hash(),
        vec![],
        &validator_key
    );
    
    // Test valid block
    assert!(blockchain.add_block(valid_block).await.is_ok());
    assert_eq!(blockchain.blocks.len(), 2);
    
    // Test invalid height
    let invalid_height_block = Block::new(
        3, // Should be 2
        blockchain.blocks[1].hash(),
        vec![],
        &validator_key
    );
    assert!(blockchain.add_block(invalid_height_block).await.is_err());
    
    // Test invalid previous hash
    let invalid_prev_hash_block = Block::new(
        2,
        "invalid".to_string(),
        vec![],
        &validator_key
    );
    assert!(blockchain.add_block(invalid_prev_hash_block).await.is_err());
}

#[test]
fn test_shared_blockchain() {
    let mut rng = OsRng;
    let mut secret = [0u8; 32];
    rng.fill_bytes(&mut secret);
    let validator_key = SigningKey::from_bytes(&secret);
    let blockchain = create_shared_blockchain(&validator_key);
    
    let locked = blockchain.lock().unwrap();
    assert_eq!(locked.blocks.len(), 1);
    assert_eq!(locked.blocks[0].header.height, 0);
}