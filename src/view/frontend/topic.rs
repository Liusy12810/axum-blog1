use askama::Template;

use crate::{
    db::paginate::Paginate,
    model::{Category, TopicArchive, TopicList, TopicDetail},
};

#[derive(Template)]
#[template(path = "frontend/topic_list.html")]
pub struct List {
    pub category_name: String,
    pub list: Paginate<Vec<TopicList>>,
    pub cats: Vec<Category>,
    pub page: u32,
    pub archives: Vec<TopicArchive>,
}

#[derive(Template)]
#[template(path = "frontend/topic_detail.html")]
pub struct Detail {
    pub cats: Vec<Category>,
    pub archives: Vec<TopicArchive>,
    pub item: TopicDetail,
}