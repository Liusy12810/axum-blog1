//!
//!

use axum::{routing::get, Router};
use serde::Deserialize;

use super::auth::{login, login_ui, logout};

pub mod index;
pub mod topic;
pub mod z_dabian;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index::index))
        .route("/auth", get(login_ui).post(login))
        .route("/logout", get(logout))
        .route("/category/:id", get(topic::list))
        .route("/topic/:id/detail", get(topic::detail))
        .route("/dabian", get(z_dabian::dabian))
}

#[derive(Deserialize)]
pub struct Args {
    pub page: Option<u32>,
}
impl Args {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(0)
    }
}
