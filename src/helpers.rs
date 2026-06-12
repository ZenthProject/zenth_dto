use rand::RngCore;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn username_to_hash_id(username: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(username.as_bytes());
    hasher.finalize().to_vec()
}

pub fn generate_nonce(size: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes
}

pub fn generate_challenge_id() -> Vec<u8> {
    generate_nonce(16)
}

pub fn generate_message_id() -> Vec<u8> {
    generate_nonce(16)
}

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before epoch")
        .as_millis() as u64
}

pub fn is_valid_hash_id(id: &[u8]) -> bool {
    id.len() == 32
}
