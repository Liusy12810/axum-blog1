//! # frontend
//! 
//! 

use askama::Template;
use axum::{response::Html, http::{StatusCode, HeaderMap, header}};
use deadpool_postgres::Client;

use crate::{Result, error::AppError, AppState};

pub mod frontend;
pub mod backend;

type HtmlView = axum::response::Html<String>;
type RedirectView = (StatusCode, HeaderMap, ());

/// # render
/// a function which render the HtmlView
/// 
fn render<T>(tmpl: T) -> Result<HtmlView> where T: Template {
    let html = tmpl.render().map_err(AppError::from)?;
    Ok(Html(html))
}

fn log_error(handler_name:&str) -> Box<dyn Fn(AppError) -> AppError> {
    let handler_name = handler_name.to_string();
    Box::new(move |err| {
        tracing::error!("Operation failed（恼）: {:#?}, {}", err, handler_name);
        err
    })
}

fn redirect(url: &str) -> Result<RedirectView> {
    let mut hm = HeaderMap::new();
    hm.append(header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, hm, ()))
}

async fn get_client(state: &AppState) -> Result<Client> {
    state.pool.get().await.map_err(AppError::from)
}