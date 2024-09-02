/*
 * Endpoints:
 * - create_contest
 * - update_contest
 * - delete_contest
 * - get_contest
 * - upload_contest_problem
 * - register_user
 * - scoreboard
 * - submissions_user_contest
 */

use anyhow::{anyhow, Result};
use axum::{extract::State, Json};
use chrono::serde::ts_milliseconds;
use primitypes::{
    consts::{
        CONTEST_MAX_DURATION_IN_SECONDS, CONTEST_MIN_DURATION_IN_SECONDS, MAX_PROBLEMS_PER_CONTEST,
    },
    contest::{Contest, ContestType},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{
    session::UserId, startup::AppState, status::ServerResponse, utils::get_current_timestamp,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct ContestForm {
    pub name: String,
    pub id: Option<u32>,
    #[serde(with = "ts_milliseconds")]
    pub start_time: chrono::DateTime<chrono::Utc>,
    #[serde(with = "ts_milliseconds")]
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub information: String,
    pub rules: String,
    pub sponsor: String,
    pub problems: Vec<u32>,
}
#[axum_macros::debug_handler]
#[tracing::instrument(name = "Insert new contest into database", skip(state))]
pub async fn post_update_or_create_contest(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    mut form: Json<ContestForm>,
) -> Result<ServerResponse, ServerResponse> {
    let time_duration = form.end_time - form.start_time;
    let problems = &mut form.problems;
    if time_duration.num_seconds() < CONTEST_MIN_DURATION_IN_SECONDS
        || time_duration.num_seconds() > CONTEST_MAX_DURATION_IN_SECONDS
    {
        return Err(ServerResponse::GenericError(anyhow!(
            "Start time and end time must be at least {} seconds and at most {} seconds",
            CONTEST_MIN_DURATION_IN_SECONDS,
            CONTEST_MAX_DURATION_IN_SECONDS
        )));
    }
    if problems.len() > MAX_PROBLEMS_PER_CONTEST {
        return Err(ServerResponse::GenericError(anyhow!(
            "Too many problems for a contest, max problems {}",
            MAX_PROBLEMS_PER_CONTEST
        )));
    }

    let problems_set: std::collections::HashSet<_> = problems.iter().collect();
    if problems_set.len() != problems.len() {
        return Err(ServerResponse::GenericError(anyhow!(
            "Duplicate problems in contest"
        )));
    }

    if !validate_problems_id(&user_id, problems.clone(), &state.pool).await? {
        return Err(ServerResponse::GenericError(anyhow!(
            "Invalid problems in contest"
        )));
    }
    if !check_new_start_date_bigger_than_current(form.start_time)? {
        return Err(ServerResponse::GenericError(anyhow!(
            "Start time must be greater than current time"
        )));
    }
    Ok(match form.id {
        Some(id) => ServerResponse::ContestId(
            update_contest_in_db(id, form.0, &state.pool, &user_id).await?,
        ),
        None => {
            ServerResponse::ContestId(create_contest_in_db(form.0, &state.pool, &user_id).await?)
        },
    })
}

pub fn check_new_start_date_bigger_than_current(
    start_time: chrono::DateTime<chrono::Utc>,
) -> Result<bool> {
    let now = get_current_timestamp()? as i64;
    Ok(start_time.timestamp_millis() >= now)
}

pub async fn create_contest_in_db(form: ContestForm, pool: &PgPool, user_id: &Uuid) -> Result<u32> {
    let contest_id = sqlx::query!(
        r#"
        INSERT INTO contest (name, author, start_date, end_date, body, problems)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        form.name,
        user_id,
        form.start_time,
        form.end_time,
        json!(primitypes::contest::ContestBody {
            information: form.information,
            rules: form.rules,
            sponsor: form.sponsor,
        }),
        &form.problems.iter().map(|x| *x as i32).collect::<Vec<_>>()
    )
    .fetch_one(pool)
    .await?
    .id;

    Ok(contest_id.try_into().unwrap())
}

pub async fn update_contest_in_db(
    contest_id: u32,
    form: ContestForm,
    pool: &PgPool,
    user_id: &Uuid,
) -> Result<u32> {
    let contest_id = sqlx::query!(
        r#"
        UPDATE contest
        SET  (name, author, start_date, end_date, body, problems) = ($1, $2, $3, $4, $5, $6)
        WHERE id = $7
        RETURNING id
        "#,
        form.name,
        user_id,
        form.start_time,
        form.end_time,
        json!(primitypes::contest::ContestBody {
            information: form.information,
            rules: form.rules,
            sponsor: form.sponsor,
        }),
        &form.problems.iter().map(|x| *x as i32).collect::<Vec<_>>(),
        contest_id as i32,
    )
    .fetch_one(pool)
    .await?
    .id;
    Ok(contest_id.try_into().unwrap())
}

pub async fn get_contest_by_id(id: u32, pool: &PgPool) -> Result<Contest> {
    let data = sqlx::query!(
        r#"
    SELECT 
        name, 
        author ,
        start_date, 
        end_date, 
        body, 
        id, 
        contest_type as "contest_type: ContestType",
        problems
    FROM contest
    WHERE id = $1
    "#,
        id as i32,
    )
    .fetch_one(pool)
    .await?;

    //let body: primitypes::contest::ContestBody = serde_json::from_value(data.4)?;
    Ok(Contest {
        name: data.name,
        id: data.id.into(),
        start_date: data.start_date,
        end_date: data.end_date,
        body: data.body.into(),
        problems: data.problems.into_iter().map(|x| x.into()).collect(),
        author: data.author,
        contest_type: data.contest_type,
    })
}

pub async fn validate_problems_id(
    user_id: &Uuid,
    problems: Vec<u32>,
    pool: &PgPool,
) -> Result<bool> {
    if problems.is_empty() {
        return Ok(true);
    }
    let mut query_builder = sqlx::QueryBuilder::new(
        "
      SELECT count(id)
      FROM problem
      WHERE id IN (
    ",
    );

    let mut separated = query_builder.separated(", ");
    for problem in problems.iter() {
        separated.push_bind(*problem as i32);
    }
    separated.push_unseparated(") ");
    query_builder.push(" AND (submitted_by  = ");
    query_builder.push_bind(user_id);
    query_builder.push(" OR is_public = true) ");
    let ans = query_builder.build().fetch_one(pool).await?;

    Ok(ans.get::<i64, _>("count") == problems.len() as i64)
}
