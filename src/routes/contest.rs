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
use askama_axum::IntoResponse;
use axum::{
    extract::{rejection::ExtensionRejection, Path, State},
    response::{Redirect, Response},
    Extension, Json,
};
use chrono::{serde::ts_milliseconds, FixedOffset};
use itertools::Itertools;
use primitypes::{
    consts::{
        CONTEST_MAX_DURATION_IN_SECONDS, CONTEST_MIN_DURATION_IN_SECONDS, MAX_PROBLEMS_PER_CONTEST,
    },
    contest::{Contest, ContestState, ContestType},
    problem::{ContestId, ProblemBody, ProblemId},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::problem::get_problem;
use crate::{
    filters::filters,
    relations::{Relation, Relations, Resource},
    rendering::WholePage,
    session::UserId,
    startup::AppState,
    status::{ResultHTML, ServerResponse},
    utils::get_current_timestamp,
    with_axum::{into_response, Template},
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
    pub frozen_time: Option<u32>,
}

#[derive(Debug, Default, Serialize, Clone, Deserialize, sqlx::FromRow, PartialEq, Eq)]
pub struct ScoreboardResponse {
    pub user_id: Uuid,
    pub username: String,
    pub problem_id: Option<i32>,
    pub penalty: i64,
    pub count: i64,
    pub problem_count: i64,
    pub fastest_time: i32,
    pub rank: i64,
    pub status: Option<String>,
}
#[derive(Template)]
#[template(path = "scoreboard.html")]
struct ScoreboardHTML {
    users: Vec<Vec<Option<ScoreboardResponse>>>,
    problems: Vec<i32>,
    problems_number: usize,
}

#[derive(Template)]
#[template(path = "contest.html", escape = "none")]
struct ContestHTML {
    contest_id: u32,
    problems: Vec<ProblemBody>,
    rules: String,
    information: String,
    name: String,
    start_time: i64,
    end_time: i64,
}

#[derive(Template)]
#[template(path = "newcontest.html")]
struct NewContestHTML;

#[derive(Template)]
#[template(path = "edit_contest.html")]
struct EditContestHTML {
    problems: String,
    contest_id: u32,
    name: String,
    start_time_date: String,
    start_time_time: String,
    end_time_date: String,
    end_time_time: String,
    frozen_time: String,
    rules: String,
    information: String,
}

#[derive(Template)]
#[template(path = "contest_ended.html")]
struct ContestEndedHTML {
    contest_id: u32,
    problems: Vec<ProblemBody>,
    rules: String,
    information: String,
    name: String,
    start_time: i64,
    end_time: i64,
}

#[derive(Template)]
#[template(path = "contest_not_started.html")]
struct ContestNotStartedHTML;

#[axum_macros::debug_handler]
pub async fn contest_get(
    Path(contest_id): Path<u32>,
    UserId(user_id): UserId,
    State(state): State<AppState>,
    mut page: Extension<WholePage>,
) -> Result<Response, ServerResponse> {
    let contest = get_contest_by_id(contest_id, &state.pool)
        .await
        .map_err(|_| ServerResponse::NotFound)?;

    let user_is_registered_in_contest = Relations::query_if(&Resource::User(user_id))
        .is(&Relation::Participant)
        .of(&Resource::Contest(contest_id.into()))
        .query_with_pool(&state.pool)
        .await?;
    match contest.status() {
        ContestState::Ended => {
            let problems = get_contest_problems_name(&contest.problems, &state.pool)
                .await
                .map_err(|_| ServerResponse::NotFound)?;
            Ok(into_response(page.with_content(&ContestEndedHTML {
                contest_id,
                problems,
                rules: contest.body.rules,
                information: contest.body.information,
                name: contest.name,
                start_time: contest.start_date.timestamp_millis(),
                end_time: contest.end_date.timestamp_millis(),
            })))
        },
        ContestState::NotStarted if user_is_registered_in_contest => {
            Ok(into_response(page.with_content(&ContestNotStartedHTML)))
        },
        ContestState::Running if user_is_registered_in_contest => {
            let problems = get_contest_problems_name(&contest.problems, &state.pool)
                .await
                .map_err(|_| ServerResponse::NotFound)?;
            Ok(into_response(page.with_content(&ContestHTML {
                contest_id,
                problems,
                rules: contest.body.rules,
                information: contest.body.information,
                name: contest.name,
                start_time: contest.start_date.timestamp_millis(),
                end_time: contest.end_date.timestamp_millis(),
            })))
        },
        // if contest is running and user is not registered
        // if contest is not started and user is not registered
        _ => Ok(Redirect::to(&format!("/api/contest/subscribe/{}", contest_id)).into_response()),
    }
}

#[axum_macros::debug_handler]
pub async fn get_new_contest(
    State(state): State<AppState>,
    mut page: Extension<WholePage>,
) -> Result<Response, ServerResponse> {
    Ok(into_response(page.with_content(&NewContestHTML)))
}

#[axum_macros::debug_handler]
pub async fn get_edit_contest(
    State(state): State<AppState>,
    Path(contest_id): Path<u32>,
    mut page: Extension<WholePage>,
) -> Result<Response, ServerResponse> {
    let contest = get_contest_by_id(contest_id, &state.pool)
        .await
        .map_err(|_| ServerResponse::NotFound)?;
    let start_time_date = contest
        .start_date
        .with_timezone(&chrono_tz::Mexico::General);
    //&FixedOffset::west_opt(6).unwrap());
    let start_time_time = start_time_date.format("%H:%M").to_string();

    let end_time_date = contest.end_date.with_timezone(&chrono_tz::Mexico::General);
    //&FixedOffset::west_opt(6).unwrap());
    let end_time_time = end_time_date.format("%H:%M").to_string();

    Ok(into_response(page.with_content(&EditContestHTML {
        problems: contest.problems.iter().map(|x| x.to_string()).join(","),
        contest_id,
        name: contest.name,
        //https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/date
        start_time_date: start_time_date.format("%Y-%m-%d").to_string(),
        start_time_time,
        end_time_date: end_time_date.format("%Y-%m-%d").to_string(),
        end_time_time,
        frozen_time: "0".to_string(),
        rules: contest.body.rules,
        information: contest.body.information,
    })))
}
#[axum_macros::debug_handler]
pub async fn get_contest_problem(
    UserId(_user_id): UserId,
    Path((contest_id, problem_letter)): Path<(u32, String)>,
    State(state): State<AppState>,
) -> Result<Response, ServerResponse> {
    let contest = get_contest_by_id(contest_id, &state.pool)
        .await
        .map_err(|_| ServerResponse::NotFound)?;
    let problems_size = contest.problems.len();
    match crate::ALPHABET.iter().position(|x| x == &problem_letter) {
        Some(idx) if idx < problems_size => {
            let problem_id = &contest.problems[idx];
            let problem = get_problem(&state.pool, problem_id.clone())
                .await
                .map_err(|_| ServerResponse::NotFound)?;

            Ok(into_response(&super::problem::ProblemHTML {
                output: problem.body.output.to_string(),
                input: problem.body.input.to_string(),
                problem: problem.body.problem.to_string(),
                contest_id,
                problem_id: problem.problem_id,
                memory_limit: problem.memory_limit,
                time_limit: problem.time_limit,
                title: format!("{} - {}", problem_letter, problem.body.name.to_string()),
                examples: problem.body.examples,
                content: "".to_string(),
                navbar: "".to_string(),
            }))
        },
        Some(_) | None => Err(ServerResponse::NotFound),
    }
    //contest.problems.iter().position(|problem|)
    //into_response(page.with_content(&ContestHTML { contest_id }))
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Insert new contest into database", skip(state))]
pub async fn post_update_or_create_contest(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    Json(mut form): Json<ContestForm>,
) -> Result<ServerResponse, ServerResponse> {
    let time_duration = form.end_time - form.start_time;
    let start_time = form.start_time;
    let end_time = form.end_time;
    let problems = &mut form.problems;
    println!("{:?}", form.id);
    println!("{:?}", start_time.timestamp_millis());
    println!("{:?}", end_time.timestamp_millis());
    if start_time >= end_time {
        return Err(ServerResponse::GenericError(anyhow!(
            "Start time must be before end time"
        )));
    }
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

    let old_contest_check = check_new_start_date_bigger_than_current(form.start_time)?;
    //    return Err(ServerResponse::GenericError(anyhow!(
    //        "Start time must be greater than current time"
    //    )));
    //}
    Ok(match form.id {
        Some(id) => {
            let contest = get_contest_by_id(id, &state.pool).await?;
            if contest.author != user_id {
                return Err(ServerResponse::GenericError(anyhow!(
                    "You are not the author of this contest"
                )));
            }
            let contest_has_started =
                contest.start_date.timestamp_millis() < get_current_timestamp()? as i64;

            println!(
                "contest {:?} form {:?}",
                contest.start_date.timestamp_millis(),
                form.start_time.timestamp_millis()
            );
            if (contest_has_started && contest.start_date != form.start_time)
                || (old_contest_check && contest.start_date != form.start_time)
            {
                return Err(ServerResponse::GenericError(anyhow!(
                    "You can't change the start time of a contest"
                )));
            }
            ServerResponse::ContestId(update_contest_in_db(id, form, &state.pool, &user_id).await?)
        },
        None => {
            if !old_contest_check {
                return Err(ServerResponse::GenericError(anyhow!(
                    "Start time must be greater than current time"
                )));
            }
            ServerResponse::ContestId(create_contest_in_db(form, &state.pool, &user_id).await?)
        },
    })
}

pub fn check_new_start_date_bigger_than_current(
    start_time: chrono::DateTime<chrono::Utc>,
) -> Result<bool> {
    let now = get_current_timestamp()? as i64;
    Ok(start_time.timestamp_millis() >= now)
}

#[axum_macros::debug_handler]
#[tracing::instrument(name = "Subscribe to a contest", skip(state))]
pub async fn post_subscribe_contest(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    Path(contest_id): Path<u32>,
) -> ResultHTML {
    Relations::create_relation(
        &Resource::User(user_id),
        &Relation::Participant,
        &Resource::Contest(contest_id.into()),
        &state.pool,
    )
    .await?;
    subscribe_user_to_contest_in_db(user_id, contest_id, &state.pool).await?;
    Ok(ServerResponse::SuccessfullySubscribedToContest(
        ContestId::from(contest_id),
    ))
}

#[derive(Template)]
#[template(path = "register_contest.html")]
struct ContestSubscription {
    contest_id: u32,
    name: String,
    information: String,
    rules: String,
    start_date: String,
    end_date: String,
}

pub async fn get_subscribe_contest(
    UserId(user_id): UserId,
    State(state): State<AppState>,
    Path(contest_id): Path<u32>,
    mut page: Extension<WholePage>,
) -> Result<Response, ServerResponse> {
    let contest = get_contest_by_id(contest_id, &state.pool)
        .await
        .map_err(|_| ServerResponse::NotFound)?;
    let start_date = contest
        .start_date
        .with_timezone(&chrono_tz::Mexico::General)
        .format("%d/%m/%Y, %H:%M:%S CST")
        .to_string();
    let end_date = contest
        .end_date
        .with_timezone(&chrono_tz::Mexico::General)
        .format("%d/%m/%Y, %H:%M:%S CST")
        .to_string();
    Ok(into_response(page.with_content(&ContestSubscription {
        contest_id,
        name: contest.name,
        information: contest.body.information,
        rules: contest.body.rules,
        start_date,
        end_date,
    })))
}
pub fn compare_chunks(a: &ScoreboardResponse, b: &ScoreboardResponse) -> bool {
    a.user_id == b.user_id
}

pub async fn subscribe_user_to_contest_in_db(
    user_id: Uuid,
    contest_id: u32,
    pool: &PgPool,
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO contest_participant (user_id, contest_id)
        VALUES ($1, $2)
        "#,
        user_id,
        contest_id as i32
    )
    .execute(pool)
    .await?;
    Ok(())
}
#[axum_macros::debug_handler]
#[tracing::instrument(name = "Get scoreboard", skip(state))]
pub async fn get_scoreboard(
    Path(contest_id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Response, ServerResponse> {
    let scoreboard = get_scoreboard_html(&state.pool, contest_id).await?;
    Ok(into_response(&scoreboard))
}

pub async fn get_scoreboard_html(pool: &PgPool, contest_id: u32) -> Result<ScoreboardHTML> {
    let data = get_scoreboard_from_db(&contest_id.into(), pool).await?;
    let contest_data = get_contest_by_id(contest_id, pool).await?;
    let chunked_by_user_id = data
        .into_iter()
        .chunk_by(|a| a.user_id)
        .into_iter()
        .map(|x| {
            let user_submissions = x.1.collect::<Vec<_>>();
            let mut idx = 0;
            let mut new_user_submissions = vec![];
            for (problem_idx, problem) in contest_data.problems.iter().enumerate() {
                if idx < user_submissions.len()
                    && user_submissions[idx].problem_id.is_some()
                    // unwrap safe because of previuos check
                    && user_submissions[idx].problem_id.unwrap() == problem.as_u32() as i32
                {
                    new_user_submissions.push(Some(user_submissions[idx].clone()));
                    idx += 1;
                } else {
                    if problem_idx == 0 {
                        new_user_submissions.push(Some(ScoreboardResponse {
                            user_id: user_submissions[0].user_id,
                            problem_id: None,
                            penalty: user_submissions[0].penalty,
                            count: 0,
                            problem_count: user_submissions[0].problem_count,
                            fastest_time: 0,
                            rank: user_submissions[0].rank,
                            status: None,
                            username: user_submissions[0].username.clone(),
                        }));
                    } else {
                        new_user_submissions.push(None);
                    }
                }
            }
            new_user_submissions
        })
        .collect::<Vec<_>>();
    Ok(ScoreboardHTML {
        users: chunked_by_user_id,
        problems: contest_data
            .problems
            .iter()
            .map(|x| x.as_u32() as i32)
            .collect(),
        problems_number: contest_data.problems.len(),
    })
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
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Contest not found"))?
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
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Contest not found"))?;

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

pub async fn get_contest_problems_name(
    contest_problems: &Vec<ProblemId>,
    pool: &PgPool,
) -> Result<Vec<ProblemBody>> {
    let data: Vec<ProblemBody> = sqlx::query!(
        r#"
    SELECT 
        body
    FROM problem
    JOIN  unnest($1::integer[]) WITH ORDINALITY t(id, ord) USING (id)
    ORDER BY t.ord
    "#,
        &contest_problems
            .iter()
            .map(|x| x.as_u32() as i32)
            .collect::<Vec<_>>()
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| serde_json::from_value(row.body).unwrap())
    .collect();

    Ok(data)
}

// https://codeforces.com/gym/105257/standings
pub async fn get_scoreboard_from_db(
    contest_id: &ContestId,
    pool: &PgPool,
) -> Result<Vec<ScoreboardResponse>> {
    let contest_data = get_contest_by_id(contest_id.as_u32(), pool).await?;
    let data: Vec<ScoreboardResponse> = sqlx::query_as(
        r#"
with mintable as (
    select
        contest_submission.user_id,
        contest_submission.problem_id,
        contest_submission.status,
        contest_submission.time,
        min_times.min
    from
        (
            select distinct
                user_id,
                problem_id,
                min(time) over (partition by user_id, problem_id)
            from contest_submission 
            where status = 'accepted'
            and contest_id = $1
        ) as min_times
    inner join contest_submission
        on
            min_times.user_id = contest_submission.user_id
            and min_times.problem_id = contest_submission.problem_id
    where
        contest_submission.time <= min_times.min
        and (contest_submission.status = 'accepted' or contest_submission.status = 'wrong_answer')
        and contest_submission.contest_id = $1
    order by contest_submission.user_id, contest_submission.time
),
 wrong_answer_count as (
    select
        distinct
        user_id,
        problem_id,
        wrong_answer_count
    from
        (
            select 
                distinct
                user_id,
                problem_id,
                sum(
                    case when status = 'accepted' then 1 else 0 end
                ) over (partition by user_id, problem_id) as acc_sum,
                count(status) over (partition by user_id, problem_id) as wrong_answer_count
            from contest_submission 
            where contest_id = $1
            and (status = 'accepted' or status = 'wrong_answer')
        ) as wrong_answer_selection
    where
     acc_sum = 0
),
penaltytable as (
    select distinct
        user_id,
        problem_id,
        sum(case when status = 'wrong_answer' then 20 else time end)
            over (partition by user_id)
        as penalty
    from mintable
    order by user_id, problem_id
),

problemcount as (
    select distinct
        user_id,
        count(problem_id)
    from mintable
    where status = 'accepted'
    group by user_id
),

submissioncount as (
    select
    distinct
        user_id,
        problem_id,
        count(problem_id) over (partition by user_id, problem_id)
    from mintable
   order by user_id 
),
latest_fastest_submission as (
    select
    distinct
        user_id,
        problem_id,
        max(time) over (partition by user_id, problem_id)
    from mintable
    where status = 'accepted'
),
general_fastest_submission as (
    select
    distinct
        user_id,
        max(time) over (partition by user_id)
    from mintable
    where status = 'accepted'
),
status_table as (
    select 
        penaltytable.user_id as user_id,
        penaltytable.problem_id as problem_id,
        'accepted' as status,
        submissioncount.count as count,
        latest_fastest_submission.max as fastest_time
    from penaltytable
        inner join submissioncount
            on penaltytable.user_id = submissioncount.user_id
            and penaltytable.problem_id = submissioncount.problem_id
        inner join latest_fastest_submission
            on penaltytable.user_id = latest_fastest_submission.user_id
            and penaltytable.problem_id = latest_fastest_submission.problem_id
    union all
    select 
        user_id,
        problem_id,
        'wrong_answer' as status,
        wrong_answer_count as count,
        0 as fastest_time
    from wrong_answer_count
),
contest_submissions as (
    select 
        distinct
        status_table.user_id,
        status_table.problem_id,
        status_table.status,
        status_table.count,
        status_table.fastest_time,
        penaltytable.penalty,
        problemcount.count as problem_count,
        general_fastest_submission.max as general_fastest_time
    from
        status_table
        inner join penaltytable
            on status_table.user_id = penaltytable.user_id
        inner join problemcount
            on status_table.user_id = problemcount.user_id
        inner join general_fastest_submission
            on status_table.user_id = general_fastest_submission.user_id
) ,
all_participants as (
    select 
        user_id,
        problem_id,
        status,
        count,
        penalty,
        problem_count,
        fastest_time,
        general_fastest_time
        from contest_submissions 
    union all
    select 
        user_id,
        null as problem_id,
        null as status,
        0 as count,
        0 as penalty,
        0 as problem_count,
        0 as fastest_time,
        0 as general_fastest_time
    from contest_participant
    where contest_id = $1
    and user_id not in (select user_id from contest_submissions)
)
select 
    ap.user_id,
    u.username,
    problem_id,
    status,
    count,
    penalty,
    problem_count,
    fastest_time,
    dense_rank() over (
        order by 
            problem_count desc, 
            penalty asc, 
            general_fastest_time asc,
            ap.user_id desc
    ) as rank
from users u,
all_participants ap
left join  unnest($2) WITH ORDINALITY t(problem_id, ord) USING (problem_id)
where u.user_id = ap.user_id
order by rank, ap.user_id, t.ord;
"#,
    )
    .bind(contest_id.as_u32() as i32)
    .bind(
        contest_data
            .problems
            .iter()
            .map(|x| x.as_u32() as i32)
            .collect::<Vec<_>>(),
    )
    .fetch_all(pool)
    .await?;
    Ok(data)
}
