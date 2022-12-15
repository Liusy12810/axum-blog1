use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{cookie, error::AppError};

pub struct Auth(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        let cookie = cookie::get_cookie(headers);
        let auth = cookie.unwrap_or("".to_string());
        if auth.is_empty() {
            Err(AppError::forbidden())
        } else {
            Ok(Auth(auth))
        }
    }
}
