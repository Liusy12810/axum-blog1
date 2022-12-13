//! # Hello-axum
//! 
//! 

pub mod error;
pub mod db;
pub mod view;
pub mod handler;
pub mod model;
pub mod form;
pub mod config;

pub type Result<T> = std::result::Result<T, error::AppError>;

/// shared state
pub struct AppState {
    /// db connection
    pub pool: deadpool_postgres::Pool,
}