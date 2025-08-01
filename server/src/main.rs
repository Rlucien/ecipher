use axum::{routing::post, Router, Json};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use shared::{KeyRequest, KeyResponse};
use std::net::SocketAddr;
use chacha20poly1305::aead::OsRng;
use chacha20poly1305::{ChaCha20Poly1305, KeyInit};

type KeyStore = Arc<Mutex<HashMap<String, Vec<u8>>>>;

async fn store_key(
    store: axum::extract::Extension<KeyStore>,
    Json(req): Json<KeyRequest>,
) -> Json<KeyResponse> {
    let mut store = store.lock().unwrap();
    // 生产环境应校验权限
    let key = match req.key_id.as_str() {
        "AES-GCM" | "ChaCha20-Poly1305" => {
            let key = ChaCha20Poly1305::generate_key(&mut OsRng);
            key.to_vec()
        },
        _ => {
            // 默认使用 32 字节随机密钥
            rand::random::<[u8; 32]>().to_vec()
        }
    };
    
    store.insert(req.key_id.clone(), key.clone());
    Json(KeyResponse { key })
}

async fn get_key(
    store: axum::extract::Extension<KeyStore>,
    Json(req): Json<KeyRequest>,
) -> Json<KeyResponse> {
    let store = store.lock().unwrap();
    let key = store.get(&req.key_id).cloned().unwrap_or_default();
    Json(KeyResponse { key })
}

#[tokio::main]
async fn main() {
    let store: KeyStore = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/store_key", post(store_key))
        .route("/get_key", post(get_key))
        .layer(axum::extract::Extension(store));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}