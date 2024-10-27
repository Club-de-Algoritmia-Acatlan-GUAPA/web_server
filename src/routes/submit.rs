use std::collections::HashMap;

use anyhow::{Context, Result};
use axum::{
    async_trait,
    extract::{FromRequest, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    RequestExt,
};
use http::header::{HeaderMap, CONTENT_TYPE};
use lapin::publisher_confirm::Confirmation;
use primitypes::{
    contest::{Language, Submission},
    problem::{ContestId, ProblemId, SubmissionId},
    submit::SubmitForm as _SubmitForm,
};
use sqlx::PgPool;

use crate::{
    session::UserId,
    startup::AppState,
    status::{ResultHTML, ServerResponse},
    utils::get_current_timestamp,
};

pub struct SubmitForm(pub _SubmitForm);
#[async_trait]
impl<S> FromRequest<S> for SubmitForm
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let boundary = parse_boundary(req.headers())
            .ok_or((StatusCode::BAD_REQUEST, "Incorrect boundaries").into_response())?;
        let stream = req.with_limited_body().into_body();
        let mut multipart = multer::Multipart::new(stream.into_data_stream(), boundary);
        let mut values = HashMap::new();
        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()).into_response())?
        {
            let name = field.name().unwrap().to_string();
            let content = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()).into_response())?;
            values.insert(name, content);
        }

        let language: Language = values
            .remove("language")
            .ok_or_else(|| "wrong field language".into_response())?
            .try_into()
            .map_err(|_| "wrong field value expected valid Language".into_response())?;

        let code = values
            .remove("code")
            .ok_or("Invalid field".into_response())?;

        let contest_id: Option<u32> = serde_json::from_str(
            values
                .remove("contest_id")
                .ok_or_else(|| "wrong field contest_id".into_response())?
                .as_str(),
        )
        .unwrap_or(None);

        let problem_id: u32 = serde_json::from_str(
            values
                .remove("problem_id")
                .ok_or_else(|| "wrong field".into_response())?
                .as_str(),
        )
        .map_err(|_| "wrong field value expected valid problem_id".into_response())?;

        return Ok(Self(_SubmitForm {
            code,
            contest_id,
            language,
            problem_id,
        }));
    }
}
#[axum_macros::debug_handler]
#[tracing::instrument(
    name = "Insert new submission into database and into rabbitmq service",
    skip(user_id, state, submission_form_tuple)
)]
pub async fn submit_post(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    submission_form_tuple: SubmitForm,
) -> ResultHTML {
    let submission_form = submission_form_tuple.0;
    let current_timestamp =
        get_current_timestamp().context("Unable to determine time right now")?;
    let contest_id = submission_form.contest_id.map(ContestId);
    let problem_id = ProblemId(submission_form.problem_id);
    let id = SubmissionId::new(
        current_timestamp,
        &problem_id,
        contest_id.as_ref(),
        &user_id,
    );

    let submission = Submission {
        problem_id,
        user_id,
        contest_id,
        code: submission_form.code.into(),
        language: submission_form.language,
        id,
    };
    let result: Result<_> = try_store_submission(&state, &submission).await;

    if result.is_err() {
        store_failed_submission(&state.pool, &submission)
            .await
            .context("Unable to store submission")?;
    };

    Ok(ServerResponse::SubmissionId(submission.id.as_u128()))
}
async fn try_store_submission(state: &AppState, submission: &Submission) -> Result<()> {
    let _ = store_submission(&state.pool, &submission).await?;
    state
        .message_broker
        .publish(&submission)
        .await?
        .await
        .map(|ack| matches!(ack, Confirmation::Ack(_)))?;
    Ok(())
}

#[tracing::instrument(name = "Store submission in submissions table", skip(pool, submission))]
pub async fn store_submission_in_contest_table(
    pool: &PgPool,
    submission: &Submission,
) -> Result<()> {
    let query = sqlx::query!(
        r#"
            INSERT INTO submission (id, user_id, code, language)
            VALUES ($1, $2, $3, $4 )
        "#,
        submission.id.as_bit_vec(),
        submission.user_id,
        &String::from_utf8_lossy(&submission.code),
        format!("{:?}", submission.language)
    )
    .execute(pool)
    .await
    .map(|_| ())?;
    Ok(query)
}

#[tracing::instrument(name = "Store submission in submissions table", skip(pool, submission))]
pub async fn store_submission(pool: &PgPool, submission: &Submission) -> Result<()> {
    let query = sqlx::query!(
        r#"
            INSERT INTO submission (id, user_id, code, language)
            VALUES ($1, $2, $3, $4 )
        "#,
        submission.id.as_bit_vec(),
        submission.user_id,
        &String::from_utf8_lossy(&submission.code),
        format!("{:?}", submission.language)
    )
    .execute(pool)
    .await
    .map(|_| ())?;
    Ok(query)
}

#[tracing::instrument(name = "Store submission in submissions table", skip(pool, submission))]
pub async fn store_failed_submission(pool: &PgPool, submission: &Submission) -> Result<()> {
    let query = sqlx::query!(
        r#"
            INSERT INTO failed_submission (id, user_id, code, language)
            VALUES ($1, $2, $3 ,$4)
        "#,
        submission.id.as_bit_vec(),
        submission.user_id,
        &String::from_utf8_lossy(&submission.code),
        format!("{:?}", submission.language)
    )
    .execute(pool)
    .await
    .map(|_| ())?;
    Ok(query)
}
fn parse_boundary(headers: &HeaderMap) -> Option<String> {
    let content_type = headers.get(CONTENT_TYPE)?.to_str().ok()?;
    multer::parse_boundary(content_type).ok()
}
