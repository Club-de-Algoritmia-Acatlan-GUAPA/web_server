use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use anyhow::Result;
use async_redis_session::RedisSessionStore;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use axum_sessions::{PersistencePolicy, SameSite, SessionHandle, SessionLayer};
use secrecy::ExposeSecret;
use tokio::sync::OwnedRwLockWriteGuard;
use uuid::Uuid;

use crate::configuration::{AppSettings, CookiesSettings, RedisSettings};

pub struct UserSession(OwnedRwLockWriteGuard<async_session::Session>);
#[derive(Clone)]
pub struct UserId(pub Uuid);

pub fn session_middleware(
    config: &RedisSettings,
    cookies: &CookiesSettings,
    app: &AppSettings,
) -> SessionLayer<RedisSessionStore> {
    let store = RedisSessionStore::new(config.uri.expose_secret().as_ref())
        .expect("Redis can't be reached");
    println!("{}", config.uri.expose_secret());
    // safari https://stackoverflow.com/questions/58525719/safari-not-sending-cookie-even-after-setting-samesite-none-secure
    let session_layer = SessionLayer::new(store, config.secret.expose_secret().as_bytes())
        .with_cookie_name(&cookies.name)
        .with_cookie_domain(&cookies.domain)
        .with_cookie_path(&cookies.path)
        .with_session_ttl(Some(Duration::from_secs(24 * 60 * 60)))
        .with_http_only(cookies.http_only)
        .with_same_site_policy(match cookies.same_site_policy.as_str() {
            "Lax" => SameSite::Lax,
            "Strict" => SameSite::Strict,
            "None" => SameSite::None,
            _ => SameSite::None,
        })
        .with_persistence_policy(match cookies.persistence_policy.as_str() {
            "ExistingOnly" => PersistencePolicy::ExistingOnly,
            "Always" => PersistencePolicy::Always,
            "ChangedOnly" => PersistencePolicy::ChangedOnly,
            _ => PersistencePolicy::Always,

        })
        .with_secure(cookies.secure);
    session_layer
}

impl UserSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&mut self) {
        self.0.regenerate();
    }

    pub fn insert_user_id(&mut self, user_id: &Uuid) -> Result<()> {
        self.0
            .insert(Self::USER_ID_KEY, user_id)
            .map_err(anyhow::Error::msg)
    }

    pub fn get_user_id(&self) -> Option<Uuid> {
        self.0.get(Self::USER_ID_KEY)
    }

    pub fn log_out(&mut self) {
        self.0.destroy()
    }
}

impl Deref for UserSession {
    type Target = OwnedRwLockWriteGuard<async_session::Session>;

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
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(session_handle): Extension<SessionHandle> =
            Extension::from_request_parts(parts, state)
                .await
                .expect("Session extension missing. Is the session layer installed?");
        let session = session_handle.write_owned().await;

        Ok(Self(session))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserId
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(uuid): Extension<UserId> = Extension::from_request_parts(parts, state)
            .await
            .expect("needs_auth function is not set try using `needs_auth`");
        Ok(uuid)
    }
}

//https://github.com/tokio-rs/axum/discussions/1829
pub async fn needs_auth<B>(
    session: UserSession,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, Response> {
    if let Some(uuid) = session.get_user_id() {
        request.extensions_mut().insert(UserId(uuid));
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            [(header::CONTENT_TYPE, "text/plain")],
            "You need to login if you want to do this operation",
        )
            .into_response())
    }
}
