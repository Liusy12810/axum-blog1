//! handler/frontend/z_dabian

use crate::{
    handler::{log_error, render, HtmlView},
    view::frontend::z_dabian::DaBian,
    Result,
};

pub async fn dabian() -> Result<HtmlView> {
    let handler_name = "frontend/index/z_dabian";
    let tmpl = DaBian {};
    render(tmpl).map_err(log_error(handler_name))
}
