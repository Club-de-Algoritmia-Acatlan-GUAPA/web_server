use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use primitypes::{
    problem::{ProblemId, SubmissionId},
    submit::{GetSubmissionId, GetSubmissionsForm, GetSubmissionsJson, GetSubmissionsSqlx},
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    filters::filters,
    session::UserId,
    startup::AppState,
    status::ServerResponse,
    with_axum::{into_response, Template},
};

pub struct SubmissionError(anyhow::Error);

impl<E> From<E> for SubmissionError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for SubmissionError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}
pub struct SubmissionPage {
    status: String,
    submission_id: String,
    language: String,
    submitted_at: String,
}
#[derive(Template)]
#[template(path = "submissions.html")]
struct SubmissionsPage {
    submissions: Vec<SubmissionPage>,
}
#[axum_macros::debug_handler]
#[tracing::instrument(name = "Get submission from a problem ID", skip(user_id, state))]
pub async fn submission_get(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    Query(submission): Query<GetSubmissionsForm>,
) -> Result<Response, ServerResponse> {
    println!("{:?}", submission);
    let res = get_submissions(
        &state.pool,
        &submission.problem_id,
        &user_id,
        submission.contest_id.as_ref(),
    )
    .await?;
    Ok(into_response(&SubmissionsPage { submissions: res }))
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Get submission from a problem ID", skip(_user_id, state))]
pub async fn submission_get_id(
    UserId(_user_id): UserId,
    State(state): State<AppState>,
    Path(submission): Path<GetSubmissionId>,
) -> Result<Json<GetSubmissionsJson>, SubmissionError> {
    //let contest_id = submission
    //    .submission_id
    //    .get_contest_id()
    //    .context("Malformed submission ID")?
    //    .is_some()
    //{
    //    return Ok(Json(json!({
    //        "status": "error",
    //        "message": "Submission not found"
    //    })));
    //}
    Ok(Json(
        get_submission_by_id(&state.pool, &submission.submission_id).await?,
    ))
}
#[tracing::instrument(name = "Get submissions from a problem_id", skip(pool))]
pub async fn get_submissions(
    pool: &PgPool,
    problem_id: &ProblemId,
    user_id: &Uuid,
    contest_id: Option<&u32>,
) -> Result<Vec<SubmissionPage>> {
    let result: Vec<_> = sqlx::query!(
        r#"
            SELECT id, status "status: String ", language
            FROM submission
            WHERE (id & $1) <> B'0'::bit(128)
            AND user_id = $2
            ORDER BY id DESC
            LIMIT 40
        "#,
        problem_id.as_submission_id_bit_vec(),
        user_id,
        //contest_id.map(|x| *x as i32)
        //if let Some(contest_id) = contest_id {
        //    *contest_id as i32
        //} else {
        //    0 as i32
        //}
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|elem| {
        let sub_id: SubmissionId = SubmissionId::from_bitvec(elem.id).unwrap();
        SubmissionPage {
            status: elem.status.to_string(),
            submission_id: sub_id.as_u128().to_string(),
            language: elem.language,
            submitted_at: sub_id.get_timestamp().unwrap_or(0).to_string(),
        }
    })
    .collect();
    Ok(result)
}

#[tracing::instrument(name = "Get submissions from a problem_id", skip(pool))]
pub async fn get_submission_by_id(
    pool: &PgPool,
    submission_id: &SubmissionId,
) -> Result<GetSubmissionsJson> {
    let elem = sqlx::query_as!(
        GetSubmissionsSqlx,
        r#"
            SELECT output, id as submission_id, status as "status: _ ", language
            FROM submission
            WHERE id = $1
            LIMIT 1
        "#,
        submission_id.as_bit_vec()
    )
    .fetch_optional(pool)
    .await?
    .map_or_else(
        || {
            return Err(anyhow::anyhow!("Submission not found"));
        },
        |elem| Ok(elem),
    )?;
    let sub_id: SubmissionId = SubmissionId::from_bitvec(elem.submission_id).unwrap();

    Ok(GetSubmissionsJson {
        output: elem.output,
        status: elem.status.to_string(),
        submission_id: sub_id.as_u128().to_string(),
        language: elem.language,
        submitted_at: sub_id.get_timestamp().unwrap_or(0),
    })
}
