use anyhow::Result;
use axum::{
    extract::{Extension, Path, Query, State},
    response::{Html, IntoResponse, Json, Redirect, Response},
};
use axum_extra::extract::WithRejection;
use primitypes::problem::{
    ProblemBody, ProblemExample, ProblemGetResponse, ProblemId, ProblemsGetResponse,
};
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
#[template(path = "problems.html")]
pub struct ProblemsPage {
    problems: Vec<ProblemsGetResponse>,
}

pub struct ProblemError(anyhow::Error);

impl<E> From<E> for ProblemError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for ProblemError {
    fn into_response(self) -> Response {
        Redirect::to("/404").into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct Param {
    id: u32,
}

#[derive(Template)]
#[template(path = "problem.html")]
pub struct ProblemHTML {
    pub output: String,
    pub input: String,
    pub problem: String,
    pub contest_id: u32,
    pub problem_id: u32,
    pub memory_limit: u32,
    pub time_limit: u32,
    pub title: String,
    pub examples: Vec<ProblemExample>,
    pub content: String,
    pub navbar: String,
}

#[axum_macros::debug_handler]
pub async fn problem_get(
    WithRejection(Path(param), _): WithRejection<Path<Param>, ProblemError>,
    mut page: Extension<WholePage>,
    State(state): State<AppState>,
) -> Result<Response, ServerResponse> {
    let Param { id } = param;
    let problem_id = ProblemId::new(id);
    let value = get_problem(&state.pool, problem_id).await?;
    Ok(into_response(page.with_content(&ProblemHTML {
        output: value.body.output.to_string(),
        input: value.body.input.to_string(),
        problem: value.body.problem.to_string(),
        contest_id: 0,
        problem_id: value.problem_id,
        memory_limit: value.memory_limit,
        time_limit: value.time_limit,
        title: value.body.name.to_string(),
        examples: value.body.examples,
        content: "".to_string(),
        navbar: "".to_string(),
    })))
}

#[axum_macros::debug_handler]
pub async fn problems_get(
    State(state): State<AppState>,
    mut page: Extension<WholePage>,
) -> Result<Response, ServerResponse> {
    Ok(into_response(page.with_content(&ProblemsPage {
        problems: get_problems(&state.pool).await?,
    })))
}

#[axum_macros::debug_handler]
pub async fn problem_static(
    WithRejection(Path(_param), _): WithRejection<Path<Param>, ProblemError>,
) -> Result<Response, ProblemError> {
    let file_path = "./static/problem.html";
    let problem_html = fs::read_to_string(file_path)
        .await
        .expect("Should have been able to read the file");
    Ok(Html(problem_html).into_response())
}

pub async fn get_problem(pool: &PgPool, id: ProblemId) -> Result<ProblemGetResponse> {
    let data: (i32, serde_json::Value, i16, i16) = sqlx::query!(
        r#"
         SELECT 
            body ,
            id,
            memory_limit,
            time_limit
         FROM problem
         WHERE id = $1
         "#,
        id.as_u32() as i32
    )
    .fetch_one(pool)
    .await
    .map(|row| (row.id, row.body, row.memory_limit, row.time_limit))?;

    let problem_body: ProblemBody = serde_json::from_str(&data.1.to_string())?;
    Ok(ProblemGetResponse {
        problem_id: data.0 as u32,
        memory_limit: data.2 as u32,
        time_limit: data.3 as u32,
        body: problem_body,
    })
}
async fn get_problems(pool: &PgPool) -> Result<Vec<ProblemsGetResponse>> {
    let data: Vec<ProblemsGetResponse> = sqlx::query!(
        r#"
         SELECT body , id
         FROM problem
         LIMIT 10
         "#,
    )
    .fetch_all(pool)
    .await?
    .iter()
    .map(|row| ProblemsGetResponse {
        problem_id: row.id as u32,
        body: serde_json::from_str(&row.body.to_string()).unwrap(),
    })
    .collect();
    Ok(data)
}
