//! # backend

use axum::{routing::get, Router};
use serde::Deserialize;

mod category;
mod index;

pub fn router() -> Router {
    let category_router = Router::new()
        .route("/", get(category::index))
        .route("/add", get(category::add_ui).post(category::add))
        .route("/del/:id", get(category::del))
        .route("/edit/:id", get(category::edit_ui).post(category::edit));
    Router::new()
        .route("/", get(index::index))
        .nest("/category", category_router)
}

#[derive(Deserialize)]
pub struct Args {
    pub msg: Option<String>,
    pub page: Option<u32>,
}
impl Args {
    pub fn msg(&self) -> String {
        self.msg.clone().unwrap_or("".to_string())
    }
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(0)
    }
}
