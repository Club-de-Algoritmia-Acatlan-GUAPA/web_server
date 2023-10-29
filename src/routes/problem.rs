use anyhow::Result;
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Json, Redirect, Response},
};
use axum_extra::extract::WithRejection;
use primitypes::problem::{ProblemBody, ProblemGetResponse, ProblemID, ProblemType};
use serde::Deserialize;
use sqlx::PgPool;
use tokio::fs;

use crate::startup::AppState;

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

#[axum_macros::debug_handler]
pub async fn problem_get(
    WithRejection(Query(param), _): WithRejection<Query<Param>, ProblemError>,
    State(state): State<AppState>,
) -> Result<Json<ProblemGetResponse>, ProblemError> {
    let Param { id } = param;
    let problem_id = if ProblemID::is_contest_problem(&id) {
        ProblemID::new(ProblemType::Contest, id)
    } else {
        ProblemID::new(ProblemType::Individual, id)
    };

    let value = get_problem(&state.pool, problem_id).await?;
    Ok(Json(value))
}

#[axum_macros::debug_handler]
pub async fn problems_get(
    //WithRejection(Query(param), _): WithRejection<Query<Param>, ProblemError>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ProblemGetResponse>>, ProblemError> {
    Ok(Json(get_problems(&state.pool).await?))
}

#[axum_macros::debug_handler]
pub async fn problem_static(
    WithRejection(Query(_param), _): WithRejection<Query<Param>, ProblemError>,
) -> Result<Response, ProblemError> {
    let file_path = "../../static/problem.html";
    let problem_html = fs::read_to_string(file_path)
        .await
        .expect("Should have been able to read the file");
    Ok(Html(problem_html).into_response())
}

#[axum_macros::debug_handler]
pub async fn problem_post(
    State(_state): State<AppState>,
    Json(_problem): Json<ProblemBody>,
) -> Result<Response, ProblemError> {
    todo!()
}

async fn get_problem(pool: &PgPool, id: ProblemID) -> Result<ProblemGetResponse> {
    let data: (i64, serde_json::Value) = sqlx::query!(
        r#"
         SELECT body , problem_id
         FROM problem
         WHERE problem_id = $1
         "#,
        id.as_u32() as i32
    )
    .fetch_one(pool)
    .await
    .map(|row| (row.problem_id, row.body))?;
    let problem_body: ProblemBody = serde_json::from_str(&data.1.to_string())?;
    Ok(ProblemGetResponse {
        problem_id: data.0 as u32,
        body: problem_body,
    })
}
async fn get_problems(pool: &PgPool) -> Result<Vec<ProblemGetResponse>> {
    let data: Vec<ProblemGetResponse> = sqlx::query!(
        r#"
         SELECT body , problem_id
         FROM problem
         LIMIT 10
         "#,
    )
    .fetch_all(pool)
    .await?
    .iter()
    .map(|row| ProblemGetResponse {
        problem_id: row.problem_id as u32,
        body: serde_json::from_str(&row.body.to_string()).unwrap(),
    })
    .collect();
    Ok(data)
}
async fn store_problem(_pool: &PgPool, _id: ProblemID) -> Result<ProblemBody> {
    todo!()
}
