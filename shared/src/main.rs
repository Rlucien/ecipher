use serde::{Deserialize, Serialize};
use aes_gcm::{Aes256Gcm, Key as AesKey, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use chacha20poly1305::{ChaCha20Poly1305, Key as ChaKey};
use chacha20poly1305::aead::{Aead as ChaAead, NewAead as ChaNewAead};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyRequest {
    pub key_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyResponse {
    pub key: Vec<u8>, // In production, keys传输需加密
}

pub fn encrypt_message(key: &[u8], plaintext: &str) -> Option<Vec<u8>> {
    // 使用前12字节作为nonce
    if key.len() >= 32 {
        let aes_cipher = Aes256Gcm::new(AesKey::from_slice(&key[..32]));
        let nonce = Nonce::from_slice(b"unique nonce"); // 12 bytes; production环境需随机
        if let Ok(ciphertext) = aes_cipher.encrypt(nonce, plaintext.as_bytes()) {
            return Some(ciphertext);
        }
    }
    
    None
}

pub fn decrypt_message(key: &[u8], ciphertext: &[u8]) -> Option<String> {
    if key.len() >= 32 {
        let aes_cipher = Aes256Gcm::new(AesKey::from_slice(&key[..32]));
        let nonce = Nonce::from_slice(b"unique nonce");
        if let Ok(plaintext) = aes_cipher.decrypt(nonce, ciphertext) {
            return String::from_utf8(plaintext).ok();
        }
    }
    
    None
}