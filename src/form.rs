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

#[derive(Deserialize)]
pub struct CreatTopic {
    pub title: String,
    pub category_id: i32,
    pub summary: String,
    pub markdown: String
}

pub type EditTopic = CreatTopic;