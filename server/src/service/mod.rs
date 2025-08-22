use crate::model::key::{CreateKeyRequest, Key, KeyResponse};
use crate::repository;
use crate::utils::encryption::encrypt_data;
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
    
    let key_id = repository::create_key(pool, &key).await?;
    
    // 获取创建的密钥
    let created_key = repository::get_key_by_id(pool, key_id).await?
        .ok_or("Failed to retrieve created key")?;
    
    Ok(KeyResponse {
        id: created_key.id.unwrap(),
        name: created_key.name,
        created_at: created_key.created_at.unwrap(),
    })
}

/// 获取密钥信息
pub async fn get_key(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<KeyResponse>, Box<dyn Error>> {
    let key = repository::get_key_by_id(pool, id).await?;
    
    match key {
        Some(key) => Ok(Some(KeyResponse {
            id: key.id.unwrap(),
            name: key.name,
            created_at: key.created_at.unwrap(),
        })),
        None => Ok(None),
    }
}

/// 删除密钥
pub async fn delete_key(
    pool: &MySqlPool,
    id: u64,
) -> Result<(), Box<dyn Error>> {
    repository::delete_key(pool, id).await?;
    Ok(())
}