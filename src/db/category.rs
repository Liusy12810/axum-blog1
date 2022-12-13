//! category

use tokio_postgres::Client;

use crate::{
    error::AppError,
    form,
    model::{Category, CatrgoryID},
    Result,
};

pub async fn create(client: &Client, frm: &form::CreateCategory) -> Result<CatrgoryID> {
    let n = super::count(
        client,
        "SELECT COUNT(*) FROM categories WHERE name=$1",
        &[&frm.name],
    )
    .await?;
    if n > 0 {
        return Err(AppError::duplicate("Duplicate category already exists"));
    }

    super::insert(
        client,
        "INSERT INTO categories (name, is_del) VALUES ($1, false) RETURNING id",
        &[&frm.name],
        "failed to create category",
    )
    .await
}

pub async fn list(client: &Client) -> Result<Vec<Category>> {
    super::query(
        client,
        "SELECT id, name, is_del FROM categories WHERE is_del=false ORDER BY id ASC LIMIT 1000",
        // "SELECT id, name, is_del FROM categories ORDER BY id ASC LIMIT 1000",
        &[],
    )
    .await
}

pub async fn del_or_restore(client: &Client, id: i32, is_del: bool) -> Result<bool> {
    let n = super::del_or_restore(client, "categories", &id, is_del).await?;
    Ok(n > 0)
}

pub async fn edit(client: &Client, frm: &form::EditCategory) -> Result<bool> {
    let n = super::count(
        client,
        "SELECT COUNT(*) FROM categories WHERE name=$1 AND id<>$2",
        &[&frm.name, &frm.id],
    )
    .await?;
    if n > 0 {
        return Err(AppError::duplicate("Duplicate category already exists"));
    }

    let n = super::execute(
        client,
        "UPDATE categories SET name=$1 WHERE id=$2",
        &[&frm.name, &frm.id],
    )
    .await?;
    Ok(n > 0)
}
pub async fn find(client: &Client, id: i32) -> Result<Category>{
    super::query_row(client, "SELECT id, name, is_del FROM categories WHERE id=$1", &[&id]).await
}