use axum::{Router, middleware::from_fn};
use axum::serve;
use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod api;
mod config;
mod model;
mod repository;
mod service;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv()?;
    
    // 初始化日志
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info".into()))
        .with(fmt::layer())
        .init();
    
    // 初始化数据库连接池
    let db_pool = config::database::init_db_pool().await?;
    
    // 构建路由
    let app = Router::new()
        .merge(api::key::routes())
        .layer(from_fn(utils::middleware::cors_middleware))
        .layer(utils::middleware::trace_layer())
        .with_state(db_pool.clone());
    
    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server running on http://{}", addr);
    
    let listener = TcpListener::bind(&addr).await?;
    serve(listener, app.into_make_service())
        .await?;
    
    Ok(())
}