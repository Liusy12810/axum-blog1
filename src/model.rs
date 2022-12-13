//! # model

use tokio_pg_mapper_derive::PostgresMapper;
use serde::Serialize;

/// catrgories
#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table="categories")]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub is_del: bool,
}

/// category ID
#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "catrgories")]
pub struct  CatrgoryID {
    pub id: i32,
}