use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;

/// Generate a default signing key for testing
pub fn default_signing_key() -> SigningKey {
    let mut rng = OsRng;
    SigningKey::generate(&mut rng)
}

/// Generate a keypair (signing key and verifying key)
pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

/// Get the verifying key from a signing key
pub fn get_verifying_key(signing_key: &SigningKey) -> VerifyingKey {
    signing_key.verifying_key()
}

/// Convert a signing key to a hex string
pub fn signing_key_to_hex(signing_key: &SigningKey) -> String {
    hex::encode(signing_key.to_bytes())
}

/// Convert a hex string to a signing key
pub fn hex_to_signing_key(hex_str: &str) -> Result<SigningKey, String> {
    let bytes = hex::decode(hex_str).map_err(|e| e.to_string())?;
    if bytes.len() != 32 {
        return Err("Invalid key length".to_string());
    }
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&bytes);
    Ok(SigningKey::from_bytes(&key_bytes))
}

/// Sign a message with a signing key
pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Signature {
    signing_key.sign(message)
}

/// Convert a signature to bytes
pub fn signature_to_bytes(signature: &Signature) -> Vec<u8> {
    signature.to_bytes().to_vec()
}

/// Convert bytes to a signature
pub fn signature_from_bytes(bytes: &[u8]) -> Result<Signature, String> {
    Signature::try_from(bytes).map_err(|e| e.to_string())
}

/// Verify a signature
pub fn verify_signature(public_key: &VerifyingKey, message: &[u8], signature: &Signature) -> bool {
    public_key.verify(message, signature).is_ok()
}

/// Generate a default verifying key for testing
pub fn default_verifying_key() -> VerifyingKey {
    let signing_key = default_signing_key();
    signing_key.verifying_key()
}

/// Convert hex string to VerifyingKey
pub fn hex_to_verifying_key(hex_str: &str) -> Result<VerifyingKey, String> {
    let bytes = hex::decode(hex_str).map_err(|e| e.to_string())?;
    if bytes.len() != 32 {
        return Err("Invalid key length".to_string());
    }
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&bytes);
    VerifyingKey::from_bytes(&key_bytes).map_err(|e| e.to_string())
}