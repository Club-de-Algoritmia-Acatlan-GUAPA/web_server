use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use primitypes::{
    problem::{ProblemID, SubmissionId},
    submit::{GetSubmissionId, GetSubmissionsForm, GetSubmissionsJson, GetSubmissionsSqlx},
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{session::UserId, startup::AppState};

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
#[axum_macros::debug_handler]
#[tracing::instrument(name = "Get submission from a problem ID", skip(user_id, state))]
pub async fn submission_get(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    Query(submission): Query<GetSubmissionsForm>,
) -> Result<Json<Vec<sqlx::types::JsonValue>>, SubmissionError> {
    let res = get_submissions(&state.pool, &submission.problem_id, &user_id).await?;
    Ok(Json(res))
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Get submission from a problem ID", skip(_user_id, state))]
pub async fn submission_get_id(
    UserId(_user_id): UserId,
    State(state): State<AppState>,
    Path(submission): Path<GetSubmissionId>,
) -> Result<Json<GetSubmissionsJson>, SubmissionError> {
    Ok(Json(
        get_submission_by_id(&state.pool, &submission.submission_id).await?,
    ))
}

#[tracing::instrument(name = "Get submissions from a problem_id", skip(pool))]
pub async fn get_submissions(
    pool: &PgPool,
    problem_id: &ProblemID,
    user_id: &Uuid,
) -> Result<Vec<sqlx::types::JsonValue>> {
    let result: Vec<_> = sqlx::query!(
        r#"
            SELECT submission_id, status "status: String ", language
            FROM submission
            WHERE (submission_id & $1) <> B'0'::bit(128)
            AND user_id = $2
            ORDER BY submission_id DESC
            LIMIT 40
        "#,
        problem_id.as_submission_id_bit_vec(),
        user_id
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|elem| {
        let sub_id: SubmissionId = SubmissionId::from_bitvec(elem.submission_id).unwrap();

        json!({
            "status": elem.status.to_string(),
            "submission_id": sub_id.as_u128().to_string(),
            "language": elem.language,
            "submitted_at": sub_id.get_timestamp().unwrap_or(0),
        })
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
            SELECT output, submission_id, status as "status: _ ", language
            FROM submission
            WHERE submission_id = $1
            LIMIT 1
        "#,
        submission_id.as_bit_vec()
    )
    .fetch_one(pool)
    .await?;
    let sub_id: SubmissionId = SubmissionId::from_bitvec(elem.submission_id).unwrap();

    Ok(GetSubmissionsJson {
        output: elem.output,
        status: elem.status.to_string(),
        submission_id: sub_id.as_u128().to_string(),
        language: elem.language,
        submitted_at: sub_id.get_timestamp().unwrap_or(0),
    })
}
