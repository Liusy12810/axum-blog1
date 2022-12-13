//! # hello-axum.rs
//!
//!
//!

use std::sync::Arc;

use axum::{Router, extract::Extension};
use axum_web_study::{
    config,
    handler::{backend, frontend}, AppState,
};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_web_study=debug");
    }
    tracing_subscriber::fmt::init();

    tracing::info!("服务已启动");

    // initialize configuration from environment
    dotenv::dotenv().ok();
    let cfg = config::Config::from_env().expect("Failed initialize configuration");
    let pool = cfg.pg.create_pool(None, tokio_postgres::NoTls).expect("Failed to build database");

    // build router and relate to db conn pool
    let frontend_routers = frontend::router();
    let backend_routers = backend::router();
    let app = Router::new()
        .nest("/", frontend_routers)
        .nest("/admin", backend_routers)
        .layer(Extension(Arc::new(AppState{ pool })));

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
