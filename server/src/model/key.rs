use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Key {
    pub id: Option<u64>,
    pub name: String,
    pub encrypted_data: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateKeyRequest {
    pub name: String,
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct KeyResponse {
    pub id: u64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}