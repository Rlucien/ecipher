use super::super::model::key::{CreateKeyRequest, Key, KeyResponse};
use super::super::repository::key_repository;
use super::super::utils::encryption::encrypt_data;
use sqlx::MySqlPool;
use std::error::Error;

pub async fn create_key(
    pool: &MySqlPool,
    request: CreateKeyRequest,
    encryption_key: &str,
) -> Result<KeyResponse, Box<dyn Error>> {
    // 加密数据
    let encrypted_data = encrypt_data(&request.data, encryption_key)?;
    
    // 创建密钥记录
    let key = Key {
        id: None,
        name: request.name,
        encrypted_data,
        created_at: None,
        updated_at: None,
    };
    
    let key_id = key_repository::create_key(pool, &key).await?;
    
    // 获取创建的密钥
    let created_key = key_repository::get_key_by_id(pool, key_id).await?
        .ok_or("Failed to retrieve created key")?;
    
    Ok(KeyResponse {
        id: created_key.id.unwrap(),
        name: created_key.name,
        created_at: created_key.created_at.unwrap(),
    })
}

// 其他业务逻辑方法...