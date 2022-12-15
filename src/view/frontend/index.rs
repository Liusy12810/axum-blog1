//! # frontend/index
//! 
//! 
//!

use askama::Template;

use crate::{model::{TopicList, Category, TopicArchive}, db::paginate::Paginate}; // src/view/frontend/index.rs


#[derive(Template)]
#[template(path="frontend/index.html")]
pub struct Index {
    pub list: Paginate<Vec<TopicList>>,
    pub page : u32,
    pub cats: Vec<Category>,
    pub archives: Vec<TopicArchive>,
}