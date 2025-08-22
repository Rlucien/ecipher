use crate::model::key::Key;
use sqlx::{MySqlPool, Result};

pub async fn create_key(pool: &MySqlPool, key: &Key) -> Result<u64> {
    let result = sqlx::query!(
        r#"
        INSERT INTO keys (name, encrypted_data, created_at, updated_at)
        VALUES (?, ?, NOW(), NOW())
        "#,
        key.name,
        key.encrypted_data
    )
    .execute(pool)
    .await?;
    
    Ok(result.last_insert_id())
}

pub async fn get_key_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Key>> {
    let key = sqlx::query_as!(Key,
        r#"
        SELECT id, name, encrypted_data, created_at, updated_at
        FROM keys
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(key)
}

/// 删除密钥
pub async fn delete_key(pool: &MySqlPool, id: u64) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM keys
        WHERE id = ?
        "#,
        id
    )
    .execute(pool)
    .await?;
    
    Ok(())
}