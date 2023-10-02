use anyhow::Result;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use primitypes::{
    problem::{ProblemID, SubmissionID},
    submit::{GetSubmissionsForm, GetSubmissionsJson, GetSubmissionsSqlx},
};
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
) -> Result<Json<Vec<GetSubmissionsJson>>, SubmissionError> {
    Ok(Json(
        get_submissions(&state.pool, &submission.problem_id, &user_id).await?,
    ))
}

#[tracing::instrument(name = "Get submissions from a problem_id", skip(pool))]
pub async fn get_submissions(
    pool: &PgPool,
    problem_id: &ProblemID,
    user_id: &Uuid,
) -> Result<Vec<GetSubmissionsJson>> {
    let result: Vec<GetSubmissionsJson> = sqlx::query_as!(
        GetSubmissionsSqlx,
        r#"
            SELECT output, submission_id, status as "status: _ ", language
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
        let sub_id: SubmissionID = SubmissionID::from_bitvec(elem.submission_id).unwrap();

        GetSubmissionsJson {
            output: elem.output,
            status: elem.status.to_string(),
            submission_id: sub_id.as_u128().to_string(),
            language: elem.language,
            submitted_at: sub_id.get_timestamp().unwrap_or(0),
        }
    })
    .collect();
    Ok(result)
}
