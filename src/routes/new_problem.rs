use anyhow::{anyhow, Result};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures::{AsyncRead, TryStreamExt};
use primitypes::problem::{ProblemForm, ProblemID, ValidationType};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{ftp::FTPClient, session::UserId, startup::AppState};

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
#[tracing::instrument(name = "Insert new problem into database", skip(state))]
pub async fn new_problem(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    form: Json<ProblemForm>,
) -> Result<Json<i32>, SubmitError> {
    let problem_id = store_problem_on_db(form.0, &state.pool, &user_id)
        .await?
        .as_u32();
    create_problem_on_ftp(ProblemID(problem_id), &state.ftp).await?;
    Ok(Json(problem_id as i32))
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Insert new test_case into database", skip(state))]
pub async fn new_test_case(
    State(state): State<AppState>,
    Path(problem_id): Path<ProblemID>,
    mut multipart: axum::extract::Multipart,
) -> Result<&'static str, SubmitError> {
    let id = Uuid::new_v4();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let stream = field
            .map_err(|_| std::io::Error::other("F"))
            .into_async_read();
        match name.as_str() {
            "input" => {
                let file_name = format!("/{}/{}.in", problem_id, id.to_string());
                store_test_case_ftp(file_name.as_str(), stream, &state.ftp).await?;
            },
            "output" => {
                let file_name = format!("/{}/{}.out", problem_id, id.to_string());
                store_test_case_ftp(file_name.as_str(), stream, &state.ftp).await?;
            },
            _ => return Err(anyhow!("Invalid field").into()),
        };
    }
    let _ = store_test_case_id_on_db(problem_id, &id, &state.pool).await?;
    Ok("Success")
}

async fn store_test_case_ftp<S>(file_name: &str, f: S, ftp: &FTPClient) -> Result<()>
where
    S: AsyncRead,
{
    ftp.store_file(file_name, &mut Box::pin(f)).await?;
    Ok(())
}

async fn create_problem_on_ftp(problem_id: ProblemID, ftp: &FTPClient) -> Result<()> {
    ftp.mkdir(problem_id.as_u32().to_string().as_str()).await?;
    Ok(())
}

async fn store_test_case_id_on_db(
    problem_id: ProblemID,
    test_case_id: &Uuid,
    pool: &PgPool,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE problem 
            SET testcases = array_append(testcases, $1) 
            WHERE id = $2
        "#,
        test_case_id.to_string(),
        problem_id.as_u32() as i32,
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn store_problem_on_db(
    form: ProblemForm,
    pool: &PgPool,
    user_id: &Uuid,
) -> Result<ProblemID> {
    let id = sqlx::query!(
        r#"
            INSERT INTO problem (
                submitted_by, 
                body, 
                checker, 
                validation,
                memory_limit,
                time_limit,
                is_public,
                testcases
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
        "#,
        user_id,
        json!(form.body),
        form.checker.unwrap_or_default(),
        form.validation as ValidationType,
        form.memory_limit as i16,
        form.time_limit as i16,
        form.is_public,
        &vec![]
    )
    .fetch_one(pool)
    .await
    .map(|row| row.id)?;
    Ok(ProblemID(id.try_into().unwrap()))
}
