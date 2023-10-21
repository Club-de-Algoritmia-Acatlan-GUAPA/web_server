use anyhow::{bail, Context, Result};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use lapin::publisher_confirm::Confirmation;
use primitypes::{
    contest::Submission,
    problem::{ContestId, ProblemID, SubmissionID},
    submit::{SubmitForm, SubmitResponse},
};
use sqlx::PgPool;

use crate::{session::UserId, startup::AppState, utils::get_current_timestamp};

pub struct SubmitError(anyhow::Error);

impl<E> From<E> for SubmitError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for SubmitError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}
#[axum_macros::debug_handler]
#[tracing::instrument(
    name = "Insert new submission into database and into rabbitmq service",
    skip(user_id, state, submission_form)
)]
pub async fn submit_post(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    Form(submission_form): Form<SubmitForm>,
) -> Result<Json<SubmitResponse>, SubmitError> {
    //
    // listo - checar que el campo de codigo no poase de 64KB  listo
    // listo - checar que este loggeado
    // listo - generar timestamp
    // listo generar id submission
    //      hora unix
    //      id_problema
    //      id_concurso
    //      user_id
    // guardar en base de datos de submissions
    // pushear a la cola -> esperar el ack y poner un timeout

    let current_timestamp =
        get_current_timestamp().context("Unable to determine time right now")?;
    let contest_id = submission_form.contest_id.map(ContestId);
    let problem_id = ProblemID(submission_form.problem_id);
    let id = SubmissionID::new(
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

    let res = SubmitResponse {
        submission_id: submission.id.as_u128().to_string(),
    };
    Ok(Json(res))
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
#[tracing::instrument(name = "Store submission in submissions table", skip(pool))]
pub async fn store_submission(pool: &PgPool, submission: &Submission) -> Result<()> {
    match sqlx::query!(
        r#"
            INSERT INTO submission (submission_id, user_id, code, language)
            VALUES ($1, $2, $3, $4 )
        "#,
        submission.id.as_bit_vec(),
        submission.user_id,
        &String::from_utf8_lossy(&submission.code),
        format!("{:?}", submission.language)
    )
    .execute(pool)
    .await
    {
        Ok(_) => return Ok(()),
        Err(e) => {
            bail!(dbg!(e))
        },
    }
}

#[tracing::instrument(name = "Store submission in submissions table", skip(pool, submission))]
pub async fn store_failed_submission(pool: &PgPool, submission: &Submission) -> Result<()> {
    match sqlx::query!(
        r#"
            INSERT INTO failed_submission (submission_id, user_id, code, language)
            VALUES ($1, $2, $3 ,$4)
        "#,
        submission.id.as_bit_vec(),
        submission.user_id,
        &String::from_utf8_lossy(&submission.code),
        format!("{:?}", submission.language)
    )
    .execute(pool)
    .await
    {
        Ok(_) => return Ok(()),
        Err(e) => {
            bail!(dbg!(e))
        },
    }
}
