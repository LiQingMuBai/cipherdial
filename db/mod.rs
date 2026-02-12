use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};
use crate::config::Config;

pub type DbPool = Pool<MySql>;

pub async fn create_pool(config: &Config) -> Result<DbPool, Box<dyn std::error::Error>> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;
    
    // 创建表（如果不存在）
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS phone_verifications (
            id CHAR(36) PRIMARY KEY,
            phone VARCHAR(20) NOT NULL,
            username VARCHAR(100) NOT NULL,
            verification_code VARCHAR(10) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            INDEX idx_phone (phone),
            INDEX idx_username (username)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
        "#,
    )
    .execute(&pool)
    .await?;
    
    Ok(pool)
}