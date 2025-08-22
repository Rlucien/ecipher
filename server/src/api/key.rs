use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Router,
};
use serde_json::json;
use sqlx::MySqlPool;

use crate::model::key::{CreateKeyRequest, KeyResponse};
use crate::service::{self as key_service};

pub fn routes() -> Router<MySqlPool> {
    Router::new()
        .route("/keys", post(handle_create_key))
        .route("/keys/:id", get(handle_get_key))
        .route("/keys/:id", delete(handle_delete_key))
}

async fn handle_create_key(
    State(pool): State<MySqlPool>,
    Json(request): Json<CreateKeyRequest>,
) -> Result<(StatusCode, Json<KeyResponse>), (StatusCode, String)> {
    // 获取加密密钥（实际应用中应从安全存储获取）
    let encryption_key = std::env::var("ENCRYPTION_KEY")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Encryption key not configured".to_string()))?;
    
    let key = key_service::create_key(&pool, request, &encryption_key)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create key: {:?}", e)))?;
    
    Ok((StatusCode::CREATED, Json(key)))
}

async fn handle_get_key(
    State(pool): State<MySqlPool>,
    Path(id): Path<u64>,
) -> Result<Json<KeyResponse>, (StatusCode, String)> {
    let key = key_service::get_key(&pool, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get key: {:?}", e)))?
        .ok_or((StatusCode::NOT_FOUND, "Key not found".to_string()))?;
    
    Ok(Json(key))
}

async fn handle_delete_key(
    State(pool): State<MySqlPool>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    key_service::delete_key(&pool, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete key: {:?}", e)))?;
    
    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Key deleted successfully" })),
    ))
}