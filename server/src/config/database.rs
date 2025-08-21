use once_cell::sync::OnceCell;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;

static DB_POOL: OnceCell<MySqlPool> = OnceCell::new();

pub async fn init_db_pool() -> Result<&'static MySqlPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    
    DB_POOL.set(pool).expect("Failed to initialize DB pool");
    Ok(DB_POOL.get().unwrap())
}

pub fn get_pool() -> &'static MySqlPool {
    DB_POOL.get().expect("DB pool not initialized")
}