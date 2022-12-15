//! # hello-axum.rs
//!
//!
//!

use std::sync::Arc;

use axum::{extract::Extension, Router, middleware::from_extractor};
use axum_web_study::{
    config,
    handler::{backend, frontend},
    middleware, AppState,
};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_web_study=debug");
    }
    tracing_subscriber::fmt::init();

    // initialize configuration from environment
    dotenv::dotenv().ok();
    let cfg = config::Config::from_env().expect("Failed initialize configuration");
    let pool = cfg
        .pg
        .create_pool(None, tokio_postgres::NoTls)
        .expect("Failed to build database");

    // build router and relate to db conn pool
    let frontend_routers = frontend::router();
    let backend_routers = backend::router().layer(from_extractor::<middleware::Auth>());
    let app = Router::new()
        .nest("/", frontend_routers)
        .nest("/admin", backend_routers)
        .layer(Extension(Arc::new(AppState { pool })));

        tracing::info!("服务已启动: {}", &cfg.web.addr);

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    tracing::info!("Process terminated. Bye~");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}