use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use std::error::Error;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_ENGINE};
use sha2::{Sha256, Digest};


// AES-256-GCM的nonce大小（12字节）
const NONCE_SIZE: usize = 12;

/// 使用AES-256-GCM加密数据
/// 
/// # 参数
/// - `data`: 要加密的原始数据
/// - `encryption_key`: 用于派生加密密钥的主密钥
/// 
/// # 返回值
/// 成功时返回Base64编码的加密数据
pub fn encrypt_data(data: &str, encryption_key: &str) -> Result<String, Box<dyn Error>> {
    // 从encryption_key派生256位密钥
    let mut hasher = Sha256::new();
    hasher.update(encryption_key.as_bytes());
    let key = hasher.finalize();
    let cipher = Aes256Gcm::new_from_slice(&key[..])?;
    
    // 生成随机nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    getrandom::getrandom(&mut nonce_bytes)?;
    // 确保nonce_bytes长度正确
    let nonce = Nonce::from_slice(&nonce_bytes[..NONCE_SIZE]);
    
    // 加密数据
    let ciphertext = cipher.encrypt(nonce, data.as_bytes())
        .map_err(|e| format!("Encryption error: {:?}", e))?;
    
    // 将nonce和密文组合并Base64编码
    let mut combined = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);
    
    Ok(BASE64_ENGINE.encode(combined))
}

/// 解密使用encrypt_data函数加密的数据
/// 
/// # 参数
/// - `encrypted_data`: Base64编码的加密数据
/// - `encryption_key`: 与加密时使用的相同主密钥
/// 
/// # 返回值
/// 成功时返回解密后的原始数据
pub fn decrypt_data(encrypted_data: &str, encryption_key: &str) -> Result<String, Box<dyn Error>> {
    // 解码Base64数据
    let combined = BASE64_ENGINE.decode(encrypted_data)?;
    
    // 分离nonce和密文
    if combined.len() < NONCE_SIZE {
        return Err("Invalid encrypted data".into());
    }
    
    let nonce_bytes = &combined[..NONCE_SIZE];
    let ciphertext = &combined[NONCE_SIZE..];
    // 确保nonce_bytes长度正确
    let nonce = Nonce::from_slice(&nonce_bytes[..NONCE_SIZE]);
    
    // 从encryption_key派生256位密钥
    let mut hasher = Sha256::new();
    hasher.update(encryption_key.as_bytes());
    let key = hasher.finalize();
    let cipher = Aes256Gcm::new_from_slice(&key[..])?;
    
    // 解密数据
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption error: {:?}", e))?;
    
    // 转换为字符串
    Ok(String::from_utf8(plaintext)?)
}