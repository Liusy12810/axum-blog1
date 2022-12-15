//! category

use tokio_postgres::{types::ToSql, Client};

use crate::{
    error::AppError,
    form,
    model::{Category, CatrgoryID},
    Result,
};

/// function to create a new category
/// I changed the code to be able to add category with the same name as the deleted ones.
pub async fn create(client: &Client, frm: &form::CreateCategory) -> Result<CatrgoryID> {
    // let n = super::count(
    //     client,
    //     "SELECT COUNT(*) FROM categories WHERE name=$1",
    //     &[&frm.name],
    // )
    // .await?;
    // if n > 0 {
    //     return Err(AppError::duplicate("Duplicate category already exists"));
    // }

    // super::insert(
    //     client,
    //     "INSERT INTO categories (name, is_del) VALUES ($1, false) RETURNING id",
    //     &[&frm.name],
    //     "failed to create category",
    // )
    // .await
    let arr = super::query::<Category>(
        client,
        "SELECT id, name, is_del FROM categories WHERE name=$1",
        &[&frm.name],
    )
    .await?;
    match arr[..] {
        [Category {
            id, is_del: true, ..
        }, ..] => {
            super::del_or_restore(client, "categories", &id, false).await?;
            Ok(CatrgoryID { id })
        }
        [Category { is_del: false, .. }, ..] => {
            Err(AppError::duplicate("Duplicate category already exists"))
        }
        [] => {
            super::insert(
                client,
                "INSERT INTO categories (name, is_del) VALUES ($1, false) RETURNING id",
                &[&frm.name],
                "failed to create category",
            )
            .await
        }
    }
}

async fn list_by_condition(
    client: &Client,
    condition: Option<&str>,
    params: Option<&[&(dyn ToSql + Sync)]>,
) -> Result<Vec<Category>> {
    let condition = condition.unwrap_or("");
    let params = params.unwrap_or(&[]);
    let sql = format!("SELECT id, name, is_del FROM categories WHERE {} ORDER BY id ASC LIMIT 1000", condition);
    super::query(client, &sql, params).await
}

/// list all the existing article.
pub async fn list(client: &Client) -> Result<Vec<Category>> {
    list_by_condition(client, Some("is_del=$1"), Some(&[&false])).await
}

/// delete or restore the category and its contained articles.
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
pub async fn find(client: &Client, id: i32) -> Result<Category> {
    super::query_row(
        client,
        "SELECT id, name, is_del FROM categories WHERE id=$1",
        &[&id],
    )
    .await
}
