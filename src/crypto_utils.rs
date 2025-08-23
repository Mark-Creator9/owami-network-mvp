use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use rand::RngCore;

pub fn generate_keypair() -> SigningKey {
    // Use the from_bytes method with random bytes
    let mut rng = OsRng;
    let mut secret_bytes = [0u8; 32];
    RngCore::fill_bytes(&mut rng, &mut secret_bytes);
    SigningKey::from_bytes(&secret_bytes.into())
}

pub fn signing_key_from_bytes(bytes: &[u8; 32]) -> Result<SigningKey, ed25519_dalek::SignatureError> {
    Ok(SigningKey::from_bytes(bytes.into()))
}

pub fn default_signing_key() -> SigningKey {
    // Create a deterministic key for testing
    let secret_bytes: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    ];
    SigningKey::from_bytes(&secret_bytes.into())
}

pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Signature {
    signing_key.sign(message)
}

pub fn verify_signature(verifying_key: &VerifyingKey, message: &[u8], signature: &Signature) -> bool {
    verifying_key.verify(message, signature).is_ok()
}

pub fn signature_to_bytes(signature: &Signature) -> Vec<u8> {
    signature.to_bytes().to_vec()
}

pub fn signature_from_bytes(bytes: &[u8]) -> Result<Signature, ed25519_dalek::SignatureError> {
    if bytes.len() != 64 {
        return Err(ed25519_dalek::SignatureError::new());
    }
    let mut sig_bytes = [0u8; 64];
    sig_bytes.copy_from_slice(bytes);
    Ok(Signature::from_bytes(&sig_bytes))
}