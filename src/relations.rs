use std::{collections::HashMap, fmt};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Path},
    http::{header, request::Parts, StatusCode},
    middleware::from_extractor,
    RequestPartsExt, Router,
};
use primitypes::problem::{ContestId, ProblemId, SubmissionId};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, QueryBuilder};
use tracing::debug;
use uuid::Uuid;

use crate::{session::UserId, startup::AppState};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "relation_type", rename_all = "snake_case")]
pub enum Relation {
    Participant,
    Owner,
    ProblemSetter,
    Member,
    Admin,
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Relation::Participant => write!(f, "participant"),
            Relation::Owner => write!(f, "owner"),
            Relation::ProblemSetter => write!(f, "problem_setter"),
            Relation::Member => write!(f, "member"),
            Relation::Admin => write!(f, "admin"),
        }
    }
}

#[derive(Default)]
pub enum Resource {
    Contest(ContestId),
    Problem(ProblemId),
    Submission(SubmissionId),
    User(Uuid),
    #[default]
    Empty,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Resource::Contest(id) => write!(f, "grn:contest/{}", id.as_u32()),
            Resource::Problem(id) => write!(f, "grn:problem/{}", id.as_u32()),
            Resource::Submission(id) => write!(f, "grn:submission/{}", id.as_u128()),
            Resource::User(id) => write!(f, "grn:user/{}", id.to_string()),
            Resource::Empty => write!(f, "grn:empty"),
        }
    }
}

pub struct Relations;

#[derive(Default)]
pub struct QueryIf<'a> {
    resource_1: Option<&'a Resource>,
    relation: Option<&'a Relation>,
    resource_2: Option<&'a Resource>,
}
impl<'a> QueryIf<'a> {
    fn is(&'a mut self, relation: &'a Relation) -> &'a mut Self {
        self.relation = Some(relation);
        self
    }

    fn of(&'a mut self, resource_2: &'a Resource) -> &'a mut Self {
        self.resource_2 = Some(resource_2);
        self
    }

    async fn query_with_pool(&self, pool: &PgPool) -> Result<bool, anyhow::Error> {
        let res_1 = if let Some(resource_1) = self.resource_1 {
            resource_1.to_string()
        } else {
            return Ok(false);
        };

        let res_2 = if let Some(resource_2) = self.resource_2 {
            resource_2.to_string()
        } else {
            return Ok(false);
        };

        let relation = if let Some(relation) = self.relation {
            relation.to_string()
        } else {
            return Ok(false);
        };
        println!("res_1: {}, res_2: {}, relation: {}", res_1, res_2, relation);
        let query = format!(
            r#"
            SELECT relation
            FROM relations
            WHERE first_grn = '{}'
            AND second_grn = '{}'
            AND relation = '{}'
        "#,
            res_1, res_2, relation
        );
        println!("query: {}", query);
        let mut query = QueryBuilder::new(&query);
        match query.build().fetch_optional(pool).await {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => {
                dbg!(&e);
                Err(e.into())
            }
        }
    }
}

impl Relations {
    pub fn query_if(resource_1: &Resource) -> QueryIf {
        QueryIf {
            resource_1: Some(resource_1),
            relation: None,
            resource_2: None,
        }
    }

    pub async fn create_relation(
        resource_1: &Resource,
        relation: &Relation,
        resource_2: &Resource,
        pool: &PgPool,
    ) -> Result<(), anyhow::Error> {
        let query = format!(
            r#"
            INSERT INTO relations (first_grn, relation, second_grn)
            VALUES ({}, {}, {})
        "#,
            resource_1.to_string(),
            relation.to_string(),
            resource_2.to_string()
        );
        let mut query = QueryBuilder::new(&query);
        query.build().execute(pool).await?;
        Ok(())
    }
}

pub struct Permission;

#[async_trait]
impl<S> FromRequestParts<S> for Permission
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let paths = Path::<HashMap<String, String>>::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        let (problem_id, user_id) = (
            paths.get("problem_id").cloned(),
            parts.extensions.get::<UserId>().cloned(),
        );

        let state = parts.extract_with_state::<AppState, _>(state).await?;

        match (problem_id, user_id) {
            (Some(problem_id), Some(user_id)) => {
                let problem_id =
                    ProblemId::try_from(problem_id).map_err(|_| StatusCode::BAD_REQUEST)?;

                let problem = Resource::Problem(problem_id);
                let user = Resource::User(user_id.0);
                let mut query = Relations::query_if(&user);

                let new_query = query.is(&Relation::ProblemSetter).of(&problem);

                return match new_query
                    .query_with_pool(&state.pool)
                    .await
                    .unwrap_or(false)
                {
                    true => Ok(Self),
                    false => Err(StatusCode::FORBIDDEN),
                };
            },
            _ => return Err(StatusCode::BAD_REQUEST),
        }
    }
}
#[async_trait]
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
