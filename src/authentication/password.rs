use anyhow::Context;
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use primitypes::user::User;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{email::Email, user::UserName};

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub enum Identifier {
    Email(Email),
    UserName(UserName),
    UserId(Uuid),
}
pub struct Credentials {
    pub identifier: Identifier,
    pub password: Secret<String>,
}
struct UserDataRetrieved {
    user_id: Uuid,
    password_hash: Secret<String>,
}
#[tracing::instrument(name = "Get stored credentials", skip(credentials, pool))]
async fn get_stored_credentials(
    credentials: &Credentials,
    pool: &PgPool,
    //) -> Result<Option<(uuid::Uuid, Secret<String>)>, anyhow::Error> {
) -> Result<Option<User>, anyhow::Error> {
    let row = match &credentials.identifier {
        Identifier::Email(email) => sqlx::query_as!(
            User,
            r#"
                SELECT 
                    user_id, 
                    username,
                    email,
                    password_hash, 
                    is_validated,
                    github,
                    website,
                    bio,
                    first_name,
                    last_name
                FROM users
                WHERE email = $1
                "#,
            email.as_ref()
        )
        .fetch_optional(pool)
        .await
        .context("Failed to performed a query to retrieve stored credentials.")?,

        Identifier::UserName(username) => sqlx::query_as!(
            User,
            r#"
                SELECT 
                    user_id, 
                    username,
                    email,
                    password_hash, 
                    is_validated,
                    github,
                    website,
                    bio,
                    first_name,
                    last_name
                FROM users
                WHERE username = $1
                "#,
            username.as_ref()
        )
        .fetch_optional(pool)
        .await
        .context("Failed to ")?,
        _ => {
            unreachable!()
        },
    };
    Ok(row)
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<User, AuthError> {
    let mut user_data = None;
    // fake hash to fail
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    if let Some(user) = get_stored_credentials(&credentials, pool).await? {
        expected_password_hash = user.password_hash.clone().into();
        user_data = Some(user);
    }

    verify_password_hash(expected_password_hash, credentials.password)?;

    user_data
        .ok_or_else(|| anyhow::anyhow!("Unknown username."))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(
    name = "Validate credentials",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Change password", skip(password, pool))]
pub async fn change_password(
    user_id: uuid::Uuid,
    password: Secret<String>,
    pool: &PgPool,
) -> Result<(), anyhow::Error> {
    let password_hash = compute_password_hash(password).context("Failed to hash password")?;
    sqlx::query!(
        r#"
        UPDATE users
        SET password_hash = $1
        WHERE user_id = $2
        "#,
        password_hash.expose_secret(),
        user_id
    )
    .execute(pool)
    .await
    .context("Failed to change user's password in the database.")?;
    Ok(())
}

pub fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(2048, 2, 1, None).unwrap(),
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)?
    .to_string();
    Ok(Secret::new(password_hash))
}
