//!

use std::sync::Arc;

use axum::extract::{Extension, Form, Query, Path};

use crate::{
    handler::{log_error, render, HtmlView, RedirectView, get_client, redirect},
    view::backend::category::{Add, Index, Edit},
    Result, AppState,
    db::category, form::{self, EditCategory}
};

use super::Args;

pub async fn add_ui() -> Result<HtmlView> {
    let handler_name = "backend/category/add_ui";
    let tmpl = Add {};
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<form::CreateCategory>
) -> Result<RedirectView> {
    let handler_name = "backend/category/add";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    category::create(&client, &frm).await.map_err(log_error(handler_name))?;
    redirect("/admin/category?msg=Successfully added category")
}

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(args): Query<Args>
) -> Result<HtmlView> {
    let handler_name = "backend/category/index";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let list = category::list(&client, false).await.map_err(log_error(handler_name))?;
    let tmpl = Index { list, msg: args.msg };
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn del(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<RedirectView> {
    let handler_name = "backend/category/del";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    category::del_or_restore(&client, id, true).await.map_err(log_error(handler_name))?;
    redirect("/admin/category?msg=Successfully deleted category")
}

pub async fn edit_ui(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>
) -> Result<HtmlView> {
    let handler_name = "backend/category/edit_ui";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    let item = category::find(&client, id).await.map_err(log_error(handler_name))?;
    let tmpl = Edit { item };
    render(tmpl).map_err(log_error(handler_name))
}

pub async fn edit(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<EditCategory>  
) -> Result<RedirectView> {
    let handler_name = "backend/category/edit_ui";
    let client = get_client(&state).await.map_err(log_error(handler_name))?;
    category::edit(&client, &frm).await.map_err(log_error(handler_name))?;
    redirect("/admin/category?msg=Successfully edited category")
}