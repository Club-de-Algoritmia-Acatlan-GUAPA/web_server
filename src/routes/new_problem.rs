use core::fmt;

use anyhow::{anyhow, Result};
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use futures::{stream::TryStreamExt, AsyncReadExt};
use primitypes::problem::{ProblemForm, ProblemID, TestCaseIdInfo, ValidationType};
use serde::{Deserialize, Serialize};
use serde_json::json;
//use sha2::{Digest, Sha256};
use sha1::{Digest, Sha1};
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    In,
    Out,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileType::In => write!(f, "in"),
            FileType::Out => write!(f, "out"),
        }
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
    Path((problem_id, filetype)): Path<(ProblemID, FileType)>,
    mut multipart: axum::extract::Multipart,
) -> Result<String, SubmitError> {
    let mut filename = None;
    let mut id = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let mut stream = field
            .map_err(|_| std::io::Error::other("F"))
            .into_async_read();

        match name.as_str() {
            "file" => {
                let mut hasher = Sha1::new();
                let mut buffer = Vec::new();
                stream.read_to_end(&mut buffer).await?;
                hasher.update(&buffer);
                let hash = hasher.finalize();
                id = format!("{:x}", hash).into();
                filename = Some(format!("/{}/{:x}.{}", problem_id, hash, filetype));
                store_test_case_ftp(filename.as_ref().unwrap(), buffer.as_slice(), &state.ftp)
                    .await?;
            },
            _ => return Err(anyhow!("Invalid field").into()),
        };
    }
    let _ = store_test_case_id_on_db(problem_id, id.as_ref().unwrap(), &state.pool).await?;
    match filename {
        Some(f) => Ok(f),
        None => Err(anyhow!("No file uploaded").into()),
    }
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Insert new test_case into database", skip(state))]
pub async fn download_test_case(
    State(state): State<AppState>,
    Path((problem_id, testcase_id, filetype)): Path<(ProblemID, String, FileType)>,
) -> impl IntoResponse {
    let filename = format!("/{}/{}.{}", problem_id, testcase_id, filetype);
    let file = get_ftp_file(filename.as_str(), &state.ftp).await;
    match file {
        Ok(file) => {
            let attach = format!("attachment; filename=\"{}\"", filename.as_str());
            let headers = [
                (header::CONTENT_TYPE, "text/toml; charset=utf-8"),
                (header::CONTENT_DISPOSITION, attach.as_str()),
            ];
            (StatusCode::OK, headers, file).into_response()
        },
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Get test case order from database", skip(state))]
pub async fn get_test_case_order(
    State(state): State<AppState>,
    Path(problem_id): Path<ProblemID>,
) -> impl IntoResponse {
    let testcases = get_test_cases_from_db(problem_id, &state.pool).await;
    match testcases {
        Ok(testcases) => Json(testcases).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Problem not found").into_response(),
    }
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Remove test case from database", skip(state))]
pub async fn remove_single_test_case(
    State(state): State<AppState>,
    Path((problem_id, testcase_id, filetype)): Path<(ProblemID, String, FileType)>,
) -> impl IntoResponse {
    let filename = format!("/{}/{}.{}", problem_id, testcase_id, filetype);
    let _ = remove_test_case_from_ftp(filename.as_str(), &state.ftp).await;
    (StatusCode::OK, "Test case removed").into_response()
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Remove test case from database", skip(state))]
pub async fn remove_whole_test_case(
    State(state): State<AppState>,
    Path((problem_id, testcase_id)): Path<(ProblemID, String)>,
) -> impl IntoResponse {
    let filename_out = format!("/{}/{}.in", problem_id, testcase_id);
    let filename_in = format!("/{}/{}.out", problem_id, testcase_id);
    let _ = remove_test_case_from_ftp(filename_in.as_str(), &state.ftp).await;
    let _ = remove_test_case_from_ftp(filename_out.as_str(), &state.ftp).await;
    (StatusCode::OK, "Test case removed").into_response()
}

async fn get_ftp_file(filename: &str, ftp: &FTPClient) -> Result<Vec<u8>> {
    let input = ftp.get_file_as_stream(filename).await?;
    Ok(input)
}

async fn store_test_case_ftp<S>(file_name: &str, f: S, ftp: &FTPClient) -> Result<()>
where
    S: futures::AsyncRead,
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
    test_case_id: &str,
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

async fn get_test_cases_from_db(
    problem_id: ProblemID,
    pool: &PgPool,
) -> Result<Vec<String>> {
    let testcases = sqlx::query!(
        r#"
            SELECT testcases
            FROM problem
            WHERE id = $1
        "#,
        problem_id.as_u32() as i32,
    )
    .fetch_one(pool)
    .await
    .map(|row| -> Vec<String> {
        //let test_cases = serde_json::from_str(&row.testcaseso_)
        row.testcases.unwrap_or_default()
        //test_cases.unwrap()
    })?;
    Ok(testcases)
}

async fn remove_test_case_from_ftp(filename: &str, ftp: &FTPClient) -> Result<()> {
    ftp.remove_file(filename).await?;
    Ok(())
}

async fn remove_test_case_from_db(
    problem_id: ProblemID,
    test_case_id: &str,
    pool: &PgPool,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE problem 
            SET testcases = array_remove(testcases, $1) 
            WHERE id = $2
        "#,
        test_case_id.to_string(),
        problem_id.as_u32() as i32,
    )
    .execute(pool)
    .await?;
    Ok(())
}
