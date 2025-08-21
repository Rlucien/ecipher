use axum::{Router, Server};
use dotenv::dotenv;
use std::net::SocketAddr;
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
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(fmt::layer())
        .init();
    
    // 初始化数据库连接池
    let db_pool = config::database::init_db_pool().await?;
    
    // 构建路由
    let app = Router::new()
        .merge(api::keys::routes())
        .with_state(db_pool);
    
    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server running on http://{}", addr);
    
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}