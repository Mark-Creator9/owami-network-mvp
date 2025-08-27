use owami_network::{
    Transaction,
    vesting::VestingManager,
    crypto_utils
};

#[test]
fn test_token_transfer() {
    // Generate test keys
    let (alice_key, alice_public_key) = crypto_utils::generate_keypair();
    let (_bob_key, bob_public_key) = crypto_utils::generate_keypair();

    let alice_addr = hex::encode(alice_public_key.to_bytes());
    let bob_addr = hex::encode(bob_public_key.to_bytes());

    // Create transaction using the constructor
    let tx = Transaction::new(
        alice_addr,
        bob_addr,
        100,
        None,
        &alice_key
    );
    
    // Use the correct verification method
    assert!(tx.verify()); 
}

#[test]
fn test_vesting_operations() {
    let manager = VestingManager::default();
    // Access the public field directly
    assert!(manager.schedules.is_empty()); 
}
