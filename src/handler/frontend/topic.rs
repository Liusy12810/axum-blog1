use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};

use crate::{
    db::{category, topic},
    handler::{
        {frontend::Args, log_error, render},
        {get_client, HtmlView},
    },
    view::frontend::topic::{Detail, List},
    AppState, Result,
};

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
    Query(args): Query<Args>,
) -> Result<HtmlView> {
    let handler_name = "frontend/topic/list";
    let page = args.page();
    let client = get_client(&state)
        .await
        .map_err(log_error(handler_name))?;
    let list = topic::list_by_cat(&client, page, id)
        .await
        .map_err(log_error(handler_name))?;
    let cats = category::list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let archives = topic::archive_list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let cat = category::find(&client, id)
        .await
        .map_err(log_error(handler_name))?;
    let tmpl = List {
        list,
        cats,
        category_name: cat.name.clone(),
        page,
        archives,
    };
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn detail(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<HtmlView> {
    let handler_name = "frontend/topic/list";
    let client = get_client(&state)
        .await
        .map_err(log_error(handler_name))?;
    let cats = category::list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let archives = topic::archive_list(&client)
        .await
        .map_err(log_error(handler_name))?;
    let item = topic::detail(&client, id).await.map_err(log_error(handler_name))?;
    let tmpl = Detail {
        cats,
        archives,
        item,
    };
    render(tmpl).map_err(log_error(handler_name))
}
