use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bit_vec::BitVec;
use chrono::DateTime;
use primitypes::{
    problem::{ProblemId, SubmissionId},
    status::Status,
    submit::{GetSubmissionId, GetSubmissionsForm, GetSubmissionsJson, GetSubmissionsSqlx},
};
use serde_json::json;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::{
    filters::filters,
    session::UserId,
    startup::AppState,
    status::{ResultHTML, ServerResponse},
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
#[derive(Default)]
pub struct SubmissionPage {
    pub status: String,
    pub submission_id: String,
    pub language: String,
    pub submitted_at: String,
    pub execution_time: String,
    pub problem_contest_id: Option<String>,
}

#[derive(Template)]
#[template(path = "submissions.html")]
pub struct SubmissionsPage {
    pub submissions: Vec<SubmissionPage>,
    pub show_problem_id: bool,
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
    Ok(into_response(&SubmissionsPage {
        submissions: res,
        show_problem_id: false,
    }))
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
    #[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
    struct SubmissionSqlx {
        id: BitVec,
        status: String,
        language: String,
        execution_time: Option<i32>,
    }
    let result: Vec<SubmissionSqlx> = match contest_id {
        Some(contest_id) => {
            sqlx::query_as!(
                SubmissionSqlx,
                r#"
                    SELECT id, status as "status: String ", language, execution_time
                    FROM submission
                    WHERE problem_id = $1
                    AND user_id = $2
                    AND contest_id = $3
                    ORDER BY id DESC
                    LIMIT 40
                "#,
                problem_id.as_u32() as i32,
                user_id,
                *contest_id as i32
            )
            .fetch_all(pool)
            .await?
        },
        None => {
            sqlx::query_as!(
                SubmissionSqlx,
                r#"
                    SELECT id, status as "status: String ", language, execution_time
                    FROM submission
                    WHERE problem_id = $1
                    AND user_id = $2
                    AND contest_id IS NULL
                    ORDER BY id DESC
                    LIMIT 40
                "#,
                problem_id.as_u32() as i32,
                user_id
            )
            .fetch_all(pool)
            .await?
        },
    };
    println!("{:?}", result);
    let new_data = result
        .into_iter()
        .map(|elem| {
            let sub_id: SubmissionId = SubmissionId::from_bitvec(elem.id).unwrap();
            let timestamp = elem.execution_time.unwrap_or(0);
            let timestamp_str = format!("{:.3}", timestamp as f64 / 1000.0);
            SubmissionPage {
                status: elem.status.to_string(),
                submission_id: sub_id.as_u128().to_string(),
                language: elem.language,
                submitted_at: DateTime::from_timestamp_millis(
                    sub_id.get_timestamp().unwrap_or(0) as i64
                )
                .unwrap_or_default()
                .with_timezone(&chrono_tz::Mexico::General)
                .format("%d/%m/%Y %H:%M:%S")
                .to_string(),
                execution_time: timestamp_str,
                ..Default::default()
            }
        })
        .collect();
    Ok(new_data)
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

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Get submission from a problem ID", skip(_user_id, state))]
pub async fn submission_get_status(
    UserId(_user_id): UserId,
    State(state): State<AppState>,
    Path(submission): Path<GetSubmissionId>,
) -> ResultHTML {
    let status = get_submission_status_from_id(&state.pool, &submission.submission_id).await?;
    Ok(ServerResponse::SubmissionStatus(status))
}

#[tracing::instrument(name = "Get submission status from submission id", skip(pool))]
pub async fn get_submission_status_from_id(
    pool: &PgPool,
    submission_id: &SubmissionId,
) -> Result<Status> {
    let elem = sqlx::query!(
        r#"
            SELECT status as "status: String "
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
        |elem| Ok(elem.status),
    )?;

    // Safe: `Status` try_from is infallible
    let status: primitypes::status::Status = elem.try_into().unwrap();

    Ok(status)
}
