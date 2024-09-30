use core::fmt;

use anyhow::{anyhow, Result};
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Extension, Json,
};
use futures::{stream::TryStreamExt, AsyncReadExt};
use primitypes::problem::{
    Checker, EditablePartsOfProblem, ProblemBody, ProblemForm, ProblemId, ValidationType,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
//use sha2::{Digest, Sha256};
use sha1::{Digest, Sha1};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    ftp::FTPClient,
    rendering::WholePage,
    session::UserId,
    startup::AppState,
    status::{FlashMessage, Messages, ResultHTML, ServerResponse, SuccessMessage},
    with_axum::{into_response, Template},
};

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

#[derive(Template)]
#[template(path = "newproblem.html")]
pub struct NewProblemPage;
#[derive(Template)]
#[template(path = "edit_problem.html")]
pub struct EditProblemPage {
    problem_id: u32,
    memory_limit: u32,
    time_limit: u32,
    is_public: bool,
    body: ProblemBody,
    checker: String,
    examples_count: usize,
    validation: ValidationType,
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Insert new problem into database", skip(page, _state))]
pub async fn new_problem_get(
    UserId(user_id): UserId,
    State(_state): State<AppState>,
    mut page: Extension<WholePage>,
) -> Response {
    into_response(page.with_content(&NewProblemPage))
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Insert new problem into database", skip(state))]
pub async fn new_problem_post(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    form: Json<ProblemForm>,
) -> ResultHTML {
    let problem_id = store_problem_on_db(form.0, &state.pool, &user_id)
        .await?
        .as_u32();
    create_problem_on_ftp(ProblemId(problem_id), &state.ftp).await?;
    Ok(ServerResponse::ProblemId(problem_id))
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Update problem in database", skip(state))]
pub async fn update_problem_post(
    State(state): State<AppState>,
    Path(problem_id): Path<ProblemId>,
    form: Json<ProblemForm>,
) -> Result<Response, ServerResponse> {
    let problem_id = update_problem_in_db(&problem_id, form.0, &state.pool).await?;
    Ok(into_response(&FlashMessage {
        message: Messages::SuccessMessage(SuccessMessage {
            message: format!("Problem {problem_id} updated successfully").as_str(),
        }),
        redirect: format!("/editproblem/{}", problem_id),
        delay_in_secs: 2.0,
    }))
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Update problem in database", skip(state, page))]
pub async fn update_problem_get(
    State(state): State<AppState>,
    Path(problem_id): Path<ProblemId>,
    mut page: Extension<WholePage>,
) -> Result<Response, ServerResponse> {
    let problem = get_problem(&state.pool, &problem_id).await?;
    let examples_count = problem.body.examples.len();
    Ok(into_response(
        page.with_content(&EditProblemPage {
            body: problem.body,
            memory_limit: problem.memory_limit as u32,
            time_limit: problem.time_limit as u32,
            examples_count,
            problem_id: problem_id.as_u32(),
            checker: problem
                .checker
                .map_or_else(|| "".to_string(), |checker| checker.checker),
            validation: problem.validation,
            is_public: problem.is_public,
        }),
    ))
}

pub async fn update_problem_in_db(
    problem_id: &ProblemId,
    form: ProblemForm,
    pool: &PgPool,
) -> Result<u32> {
    let problem_id = sqlx::query!(
        r#"
        UPDATE problem
        SET  (body, checker, validation, memory_limit, time_limit, is_public) = ($1, $2, $3, $4, $5, $6)
        WHERE id = $7
        RETURNING id
        "#,
        json!(form.body),
        form.checker.unwrap_or_default(),
        form.validation as ValidationType,
        form.memory_limit as i16,
        form.time_limit as i16,
        form.is_public,
        problem_id.as_u32() as i32,
    )
    .fetch_one(pool)
    .await?
    .id;
    Ok(problem_id.try_into().unwrap())
}

pub async fn get_problem(pool: &PgPool, id: &ProblemId) -> Result<EditablePartsOfProblem> {
    let data: EditablePartsOfProblem = sqlx::query!(
        r#"
         SELECT 
            body ,
            id,
            memory_limit,
            time_limit,
            checker,
            validation as "validation: ValidationType",
            is_public,
            testcases
         FROM problem
         WHERE id = $1
         "#,
        id.as_u32() as i32
    )
    .fetch_one(pool)
    .await
    .map(|row| EditablePartsOfProblem {
        memory_limit: row.memory_limit as u16,
        time_limit: row.time_limit as u16,
        body: serde_json::from_str(&row.body.to_string()).unwrap(),
        checker: row
            .checker
            .map_or_else(|| None, |checker| Some(Checker { checker })),
        validation: row.validation as _,
        is_public: row.is_public,
        test_cases: row.testcases,
    })?;

    Ok(data)
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Insert new test_case into database", skip(state))]
pub async fn new_test_case(
    State(state): State<AppState>,
    Path((problem_id, filetype)): Path<(ProblemId, FileType)>,
    mut multipart: axum::extract::Multipart,
) -> Result<String, SubmitError> {
    let mut filename = None;
    let mut id = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let mut stream = field
            .map_err(|_| std::io::Error::other("Unable to converto to Stream"))
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
    Path((problem_id, testcase_id, filetype)): Path<(ProblemId, String, FileType)>,
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
    Path(problem_id): Path<ProblemId>,
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
    Path((problem_id, testcase_id, filetype)): Path<(ProblemId, String, FileType)>,
) -> impl IntoResponse {
    let filename = format!("/{}/{}.{}", problem_id, testcase_id, filetype);
    let _ = remove_test_case_from_ftp(filename.as_str(), &state.ftp).await;
    (StatusCode::OK, "Test case removed").into_response()
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Remove test case from database", skip(state))]
pub async fn remove_whole_test_case(
    State(state): State<AppState>,
    Path((problem_id, testcase_id)): Path<(ProblemId, String)>,
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

async fn create_problem_on_ftp(problem_id: ProblemId, ftp: &FTPClient) -> Result<()> {
    ftp.mkdir(problem_id.as_u32().to_string().as_str()).await?;
    Ok(())
}

async fn store_test_case_id_on_db(
    problem_id: ProblemId,
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
) -> Result<ProblemId> {
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
    Ok(ProblemId(id.try_into().unwrap()))
}

async fn get_test_cases_from_db(problem_id: ProblemId, pool: &PgPool) -> Result<Vec<String>> {
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
        row.testcases
        //test_cases.unwrap()
    })?;
    Ok(testcases)
}

async fn remove_test_case_from_ftp(filename: &str, ftp: &FTPClient) -> Result<()> {
    ftp.remove_file(filename).await?;
    Ok(())
}

async fn remove_test_case_from_db(
    problem_id: ProblemId,
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
