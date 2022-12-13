//! 
//! 

use axum::{Router, routing::get};

pub mod index;
pub mod z_dabian;

pub fn router() -> Router {
    Router::new().route("/", get(index::index))
        .route("/dabian", get(z_dabian::dabian))
}