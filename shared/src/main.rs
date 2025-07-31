use serde::{Deserialize, Serialize};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or any other symmetric encryption
use aes_gcm::aead::{Aead, NewAead};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyRequest {
    pub key_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyResponse {
    pub key: Vec<u8>, // In production, keys传输需加密
}

pub fn encrypt_message(key: &[u8], plaintext: &str) -> Option<Vec<u8>> {
    let cipher = Aes256Gcm::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(b"unique nonce"); // 12 bytes; production环境需随机
    cipher.encrypt(nonce, plaintext.as_bytes()).ok()
}

pub fn decrypt_message(key: &[u8], ciphertext: &[u8]) -> Option<String> {
    let cipher = Aes256Gcm::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(b"unique nonce");
    cipher.decrypt(nonce, ciphertext).ok().and_then(|pt| String::from_utf8(pt).ok())
}