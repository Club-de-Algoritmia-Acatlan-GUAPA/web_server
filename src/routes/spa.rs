use anyhow::Result;
use axum::{
    extract::{Extension, OriginalUri, Path, Query, State},
    response::{Html, IntoResponse, Json, Redirect, Response},
};
use axum_extra::extract::WithRejection;
use primitypes::problem::{ProblemBody, ProblemGetResponse, ProblemId};
use serde::Deserialize;
use sqlx::PgPool;
use tokio::fs;

use crate::{
    rendering::WholePage,
    startup::AppState,
    status::ServerResponse,
    with_axum::{into_response, Template},
};

#[derive(Template)]
#[template(path = "spa.html")]
pub struct SpaPage {
    page: String,
}

#[axum_macros::debug_handler]
pub async fn spa_get(
    mut page: Extension<WholePage>,
    OriginalUri(uri): OriginalUri,
) -> Result<Response,Response> {
    let path = format!("{}.html", uri.path());
    Ok(into_response(page.with_content(SpaPage { page: path })))
}
