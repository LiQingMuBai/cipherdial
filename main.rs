use axum::{
    Router,
    routing::{get, post, put},  // 添加 put
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod model;
mod service;
mod controller;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "phone_verification=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let config = config::Config::from_env()?;
    tracing::info!("配置加载成功");

    // 创建数据库连接池
    let pool = db::create_pool(&config).await?;
    tracing::info!("数据库连接成功");

    // 创建服务
    let verification_service = service::VerificationService::new(pool);

    // 创建应用状态
    let state = Arc::new(controller::AppState {
        verification_service,
    });

    // 构建路由
    let app = Router::new()
        .route("/health", get(controller::health_check))
        // 使用新的 create_or_update 方法，支持 POST 和 PUT
        .route("/api/verifications", post(controller::create_or_update_verification))
        .route("/api/verifications", put(controller::create_or_update_verification))
        .route("/api/verifications", get(controller::get_verifications))
        // 根据用户名获取手机号
        .route("/api/phone/:username", get(controller::get_phone_by_username_path))
        .route("/api/phone", post(controller::get_phone_by_username_json))
        // 根据用户名获取所有记录
        .route("/api/verifications/:username", get(controller::get_verifications_by_username))
        .with_state(state);

    // 创建 TCP 监听器
    let addr = config.server_addr();
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("服务器启动于 http://{}", addr);

    // 启动服务器
    axum::serve(listener, app).await?;

    Ok(())
}