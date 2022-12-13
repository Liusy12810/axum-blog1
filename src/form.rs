//! # form

use serde::Deserialize;


#[derive(Deserialize)]
pub struct CreateCategory {
    pub name: String,
}

#[derive(Deserialize)]
pub struct EditCategory {
    pub id : i32,
    pub name: String,
}