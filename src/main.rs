use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

mod api;
mod controllers;
mod db;
mod middleware;
mod routers;
mod tools;

#[tokio::main]
async fn main() {
    // 初始化日志记录器

    let filter = EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // 初始化数据库连接
    let _ = db::mysql_connect().await;

    // 初始化路由00
    let app = routers::init().await.into_make_service();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8800").await.unwrap();
    println!("server port on {}", "0.0.0.0:8800");
    // 启动服务
    axum::serve(listener, app).await.unwrap();

    // 关闭数据库连接
    let _ = db::mysql_disconnect().await;
}
