//! topics (article) are defined here
//!

use std::time;

use tokio_postgres::{types::ToSql, Client};

use crate::{
    form, md,
    model::{TopicArchive, TopicEditData, TopicID, TopicList, TopicDetail},
    Result,
};

use super::{paginate::Paginate, DEFAULT_PAGE_SIZE};

pub async fn create(client: &Client, frm: &form::CreateTopic) -> Result<TopicID> {
    let html = md2html(&frm.markdown);
    let dateline = time::SystemTime::now();
    super::insert(
        client,
        "INSERT INTO topics (title, category_id, summary, markdown, html, hit, dateline, is_del) VALUES ($1, $2, $3, $4, $5, 0, $6, false) RETURNING id",
        &[
            &frm.title,
            &frm.category_id,
            &frm.summary,
            &frm.markdown,
            &html,
            &dateline,
        ],
        "failed creating new Topic",
    )
    .await
}

async fn list_by_condition(
    client: &Client,
    page: u32,
    condition: Option<&str>,
    params: Option<&[&(dyn ToSql + Sync)]>,
) -> Result<Paginate<Vec<TopicList>>> {
    let sql = "SELECT id, title, category_id, summary, hit, dateline, is_del, category_name FROM v_topic_cat_list WHERE is_del=false %CONDITION% ORDER BY id DESC";
    let condition = match condition {
        Some(c) => format!("AND {}", c),
        None => "".to_string()
    };
    let sql = sql.replace("%CONDITION%", &condition);
    let sql = format!("{} LIMIT {} OFFSET {}", sql, DEFAULT_PAGE_SIZE, DEFAULT_PAGE_SIZE as u32 * page);
    let count_sql = "SELECT COUNT(*) FROM v_topic_cat_list WHERE is_del=false %CONDITION%";
    let count_sql = count_sql.replace("%CONDITION%", &condition);
    let params = params.unwrap_or(&[]);
    super::pagination(client, &sql, &count_sql, params, page).await
}

pub async fn list(client: &Client, page: u32) -> Result<Paginate<Vec<TopicList>>> {
    list_by_condition(client, page, None, None).await
}

pub async fn list_by_cat(client: &Client, page: u32, cat_id: i32) -> Result<Paginate<Vec<TopicList>>> {
    list_by_condition(client, page, Some("category_id=$1"), Some(&[&cat_id])).await
}

pub async fn detail(client: &Client, id: i64) -> Result<TopicDetail> {
    super::execute(client, "UPDATE topics SET hit=hit + 1 WHERE id=$1", &[&id]).await?;
    let sql = "SELECT id, title, category_id, html, hit, dateline, is_del, category_name FROM v_topic_cat_detail WHERE is_del=false AND id=$1 LIMIT 1";
    super::query_row(client, sql, &[&id]).await
}

pub async fn update(client: &Client, frm: &form::EditTopic, id: i64) -> Result<bool> {
    let html = md2html(&frm.markdown);
    let sql = "UPDATE topics SET title=$1, category_id=$2, summary=$3, markdown=$4, html=$5 WHERE id=$6";
    let n = super::execute(
        client,
        sql,
        &[
            &frm.title,
            &frm.category_id,
            &frm.summary,
            &frm.markdown,
            &html,
            &id,
        ],
    )
    .await?;
    Ok(n > 0)
}

pub async fn find_to_edit(client: &Client, id: i64) -> Result<TopicEditData> {
    super::query_row(
        client,
        "SELECT id, title, category_id, summary, markdown FROM topics WHERE id=$1 LIMIT 1",
        &[&id],
    )
    .await
}

pub async fn del_or_restore(client: &Client, id: i64, is_del: bool) -> Result<bool> {
    let n = super::del_or_restore(client, "topics", &id, is_del).await?;
    Ok(n > 0)
}

fn md2html(markdown: &str) -> String {
    md::to_html(markdown)
}

pub async fn archive_list(client: &Client) -> Result<Vec<TopicArchive>> {
    let sql = "SELECT
       to_char(DATE_TRUNC('month', dateline), 'YYYY???MM???') AS  dateline FROM topics GROUP BY to_char(DATE_TRUNC('month', dateline), 'YYYY???MM???')";
    super::query(client, sql, &[]).await
}
