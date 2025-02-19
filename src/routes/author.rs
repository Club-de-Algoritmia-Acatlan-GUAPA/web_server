use anyhow::anyhow;
use axum::{
    extract::{Extension, State},
    response::Response,
};
use primitypes::{
    contest::{Contest, ContestType},
    problem::ProblemsGetResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    rendering::WholePage,
    session::UserId,
    startup::AppState,
    status::ServerResponse,
    with_axum::{into_response, Template},
};

#[derive(Template)]
#[template(path = "author_dashboard.html")]
struct AuthorDashboardHTML {
    problems: Vec<ProblemsGetResponse>,
    contests: Vec<Contest>,
}

#[derive(Template)]
#[template(path = "contests.html")]
struct ContestsHTML {
    contests: Vec<Contest>,
}

#[axum_macros::debug_handler]
pub async fn get_author_dashboard(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    mut page: Extension<WholePage>,
) -> Result<Response, ServerResponse> {
    let contests = get_contests_by_user_id(&state.pool, user_id).await?;
    let problems = get_problems(&state.pool, user_id).await?;
    Ok(into_response(
        page.with_content(&AuthorDashboardHTML { problems, contests }),
    ))
}

#[axum_macros::debug_handler]
pub async fn get_contests(
    State(state): State<AppState>,
    mut page: Extension<WholePage>,
) -> Result<Response, ServerResponse> {
    let contests = get_contests_from_db(&state.pool).await?;
    Ok(into_response(page.with_content(&ContestsHTML { contests })))
}

async fn get_contests_from_db(pool: &PgPool) -> anyhow::Result<Vec<Contest>> {
    let data: Vec<Contest> = sqlx::query!(
        r#"
    SELECT 
        name, 
        author ,
        start_date, 
        end_date, 
        body, 
        id, 
        contest_type as "contest_type: ContestType",
        problems,
        frozen_time,
        is_frozen
    FROM contest
    "#,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| Contest {
        name: row.name,
        id: row.id.into(),
        start_date: row.start_date,
        end_date: row.end_date,
        body: row.body.into(),
        problems: row.problems.into_iter().map(|x| x.into()).collect(),
        author: row.author,
        contest_type: row.contest_type,
        frozen_time: row.frozen_time.unwrap_or(0),
        is_frozen: row.is_frozen,
    })
    .collect();
    Ok(data)
}

async fn get_problems(pool: &PgPool, user_id: Uuid) -> anyhow::Result<Vec<ProblemsGetResponse>> {
    let data: Vec<ProblemsGetResponse> = sqlx::query!(
        r#"
         SELECT body , id
         FROM problem
         WHERE submitted_by = $1
         LIMIT 20
         "#,
        user_id
    )
    .fetch_all(pool)
    .await?
    .iter()
    .map(|row| ProblemsGetResponse {
        problem_id: row.id as u32,
        body: serde_json::from_str(&row.body.to_string()).unwrap(),
    })
    .collect();
    Ok(data)
}

pub async fn get_contests_by_user_id(pool: &PgPool, user_id: Uuid) -> anyhow::Result<Vec<Contest>> {
    let data: Vec<Contest> = sqlx::query!(
        r#"
    SELECT 
        name, 
        author ,
        start_date, 
        end_date, 
        body, 
        id, 
        contest_type as "contest_type: ContestType",
        is_frozen,
        frozen_time,
        problems
    FROM contest
    WHERE author = $1
    "#,
        user_id
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| Contest {
        name: row.name,
        id: row.id.into(),
        start_date: row.start_date,
        end_date: row.end_date,
        body: row.body.into(),
        problems: row.problems.into_iter().map(|x| x.into()).collect(),
        author: row.author,
        contest_type: row.contest_type,
        frozen_time: row.frozen_time.unwrap_or(0),
        is_frozen: row.is_frozen,
    })
    .collect();

    Ok(data)
}
