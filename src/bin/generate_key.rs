use owami_network::crypto_utils;

fn main() {
    let (signing_key, verifying_key) = crypto_utils::generate_keypair();
    let private_key_hex = crypto_utils::signing_key_to_hex(&signing_key);
    let public_key_hex = hex::encode(verifying_key.to_bytes());
    
    println!("Private Key (hex): {}", private_key_hex);
    println!("Public Key (hex): {}", public_key_hex);
    println!("");
    println!("Use this private key in your test script for transfer operations.");
}