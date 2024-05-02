use anyhow::{anyhow, Result};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::{Postgres, Transaction};
use tokio::fs;
use uuid::Uuid;

use crate::{
    authentication::password::{validate_credentials, Credentials, Identifier},
    domain::{email::Email, new_subscriber::NewSubscriber, user::UserName},
    email_client::EmailClient,
    session::UserSession,
    startup::AppState,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FormData {
    identifier: String,
    password: Secret<String>,
}

pub struct SignUpError(anyhow::Error);

impl<E> From<E> for SignUpError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for SignUpError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

#[axum_macros::debug_handler]
pub async fn login_get(session: UserSession) -> Response {
    let file_path = "./static/login.html";
    let login_html = fs::read_to_string(file_path)
        .await
        .expect("Should have been able to read the file");

    match session.get_user_id().await {
        Ok(Some(_)) => Redirect::to("/problems").into_response(),
        _ => Html(login_html).into_response(),
    }
}
#[axum_macros::debug_handler]
pub async fn login_post(
    mut session: UserSession,
    State(state): State<AppState>,
    Form(form): Form<FormData>,
) -> Result<Response, SignUpError> {
    if session.get_user_id().await?.is_some() {
        return Ok("Already logged in".into_response());
    };

    let identifier: Identifier = if let Ok(email) = form.identifier.parse::<Email>() {
        Identifier::Email(email)
    } else if let Ok(username) = form.identifier.parse::<UserName>() {
        Identifier::UserName(username)
    } else {
        return Err(anyhow!("Invalid username or email").into());
    };
    let credentials = Credentials {
        identifier,
        password: form.password,
    };

    match validate_credentials(credentials, &state.pool).await {
        Ok(user_id) => {
            session.renew().await;
            if session.insert_user_id(&user_id).await.is_err() {
                return Err(anyhow!("Unable to register the session").into())
            }
        },
        Err(e) => return Err(anyhow!(e).into()),
    };
    Ok((
        [
            (http::header::ACCESS_CONTROL_ALLOW_ORIGIN, state.base_url),
            (
                http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                true.to_string(),
            ),
        ],
        "Succesful Login",
    )
        .into_response())
}

#[tracing::instrument(name = "Insert new subscriber", skip(new_subscriber, transaction))]
pub async fn insert_new_subscriber(
    new_subscriber: &NewSubscriber,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<Uuid, sqlx::Error> {
    // user_id is auto generated by the database on insertion
    let user_id: Uuid = sqlx::query!(
        r#"
         INSERT INTO users ( email, password_hash, username )
         VALUES ( $1, $2, $3 )
         RETURNING user_id
         "#,
        new_subscriber.email.as_ref(),
        new_subscriber.password_hash.expose_secret(),
        new_subscriber.username.as_ref(),
    )
    .fetch_one(&mut **transaction)
    .await
    .map(|row| row.user_id)?;

    Ok(user_id)
}

#[tracing::instrument(name = "Store confirmation token", skip(token, transaction))]
pub async fn store_confirmation_token(
    token: &String,
    user_id: Uuid,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
         INSERT INTO confirmation_tokens ( user_id, confirmation_token )
         VALUES ( $1, $2 )
         "#,
        user_id,
        token
    )
    .execute(&mut **transaction)
    .await?;

    Ok(())
}

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, new_subscriber, base_url, subscription_token)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: NewSubscriber,
    base_url: &str,
    subscription_token: &str,
) -> Result<()> {
    let confirmation_link = format!(
        "{}/auth/confirm?confirmation_token={}",
        base_url, subscription_token
    );
    let plain_body = format!(
        "Bienvenido al Juez Guapa, estamos muy emocionados de que te unieras :D!.\n Por favor ingresa al siguiente {} Para confirmar tu subscripción.",
        confirmation_link
    );
    let html_body = format!(
        "<h1>Bienvenido al Juez Guapa, estamos muy emocionados de que te unieras!</h1><br /> <h2>Por favor ingresa al siguiente <a href=\"{}\">link</a> para confirmar tu subscripción.</h2>",
        confirmation_link
    );
    email_client
        .send_email(
            &new_subscriber.email,
            "Bienvenido++",
            &html_body,
            &plain_body,
        )
        .await
}
