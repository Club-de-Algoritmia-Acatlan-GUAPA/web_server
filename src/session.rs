use std::ops::{Deref, DerefMut};

use anyhow::Result;
use async_redis_session::RedisSessionStore;
use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::{header, request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use secrecy::ExposeSecret;
use time::Duration;
use tower_sessions::{
    cookie::SameSite, fred::prelude::RedisPool, Expiry, RedisStore, Session, SessionManagerLayer,
};
use uuid::Uuid;

use crate::configuration::{AppSettings, CookiesSettings, RedisSettings};

pub struct UserSession(Session);

#[derive(Clone)]
pub struct UserId(pub Uuid);

pub fn session_middleware(
    redis_pool: RedisPool,
    config: &RedisSettings,
    cookies: &CookiesSettings,
    app: &AppSettings,
) -> SessionManagerLayer<RedisStore<RedisPool>> {
    let store = RedisSessionStore::new(config.uri.expose_secret().as_ref())
        .expect("Redis can't be reached");
    println!("{}", config.uri.expose_secret());
    let session_store = RedisStore::new(redis_pool);

    let session_layer = SessionManagerLayer::new(session_store)
        .with_name(&cookies.name)
        .with_domain(cookies.domain.clone())
        .with_path(cookies.path.clone())
        .with_expiry(Expiry::OnInactivity(Duration::seconds(24 * 60 * 60)))
        .with_http_only(cookies.http_only)
        .with_same_site(match cookies.same_site_policy.as_str() {
            "Lax" => SameSite::Lax,
            "Strict" => SameSite::Strict,
            "None" => SameSite::None,
            _ => SameSite::Strict,
        })
        .with_secure(cookies.secure);
    session_layer
}

impl UserSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub async fn renew(&mut self) {
        let _ = self.0.cycle_id().await;
    }

    pub async fn insert_user_id(&mut self, user_id: &Uuid) -> Result<()> {
        self.0
            .insert(Self::USER_ID_KEY, user_id)
            .await
            .map_err(anyhow::Error::msg)
    }

    pub async fn get_user_id(&self) -> Result<Option<Uuid>> {
        Ok(self.0.get::<Uuid>(Self::USER_ID_KEY).await?)
    }

    pub async fn log_out(&mut self) -> Result<()> {
        self.0.flush().await?;
        Ok(())
    }
}

impl Deref for UserSession {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UserSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserSession
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;
        Ok(Self(session))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserId
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(uuid): Extension<UserId> = Extension::from_request_parts(parts, state)
            .await
            .expect("needs_auth function is not set try using `needs_auth`");
        Ok(uuid)
    }
}

//https://github.com/tokio-rs/axum/discussions/1829
pub async fn needs_auth(
    session: UserSession,
    mut request: Request,
    next: Next,
) -> Result<Response, Response> {
    if let Ok(Some(uuid)) = session.get_user_id().await {
        request.extensions_mut().insert(UserId(uuid));
        let response = next.run(request).await;
        return Ok(response)
    }
    Err((
        StatusCode::UNAUTHORIZED,
        [(header::CONTENT_TYPE, "text/plain")],
        "You need to login if you want to do this operation",
    )
        .into_response())
}
