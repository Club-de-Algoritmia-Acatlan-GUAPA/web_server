use std::str::FromStr;

use anyhow::{anyhow, Result};
use primitypes::consts::{USERNAME_FORBIDDEN_CHARACTERS, USERNAME_MAX_SIZE, USERNAME_MIN_SIZE};
use sqlx::PgPool;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserName(String);

impl FromStr for UserName {
    type Err = anyhow::Error;

    fn from_str(maybe_user: &str) -> Result<Self, Self::Err> {
        let size = maybe_user.graphemes(true).count();

        let is_valid_size = (USERNAME_MIN_SIZE..=USERNAME_MAX_SIZE).contains(&size);
        let is_empty_or_whitespace = maybe_user.trim().is_empty();
        let contain_forbidden_chars = maybe_user
            .chars()
            .any(|c| USERNAME_FORBIDDEN_CHARACTERS.contains(&c));

        if !is_valid_size {
            return Err(anyhow!(
                "Invalid username size must be between {} and {}",
                USERNAME_MIN_SIZE,
                USERNAME_MAX_SIZE
            ));
        }
        if is_empty_or_whitespace {
            return Err(anyhow!("Username cannot be empty or whitespace"));
        }

        if contain_forbidden_chars {
            return Err(anyhow!(
                "Username cannot contain the following characters: {:?}",
                USERNAME_FORBIDDEN_CHARACTERS
            ));
        }
        Ok(Self(maybe_user.to_string()))
    }
}
impl UserName {
    pub async fn get_user_id_with_username(
        &self,
        pool: &PgPool,
    ) -> Result<Option<Uuid>, anyhow::Error> {
        let user_id: Option<Uuid> = sqlx::query!(
            r#"
            SELECT user_id
            FROM users
            WHERE username = $1
            "#,
            self.0
        )
        .fetch_optional(pool)
        .await?
        .map(|row| row.user_id);
        Ok(user_id)
    }
}

impl AsRef<str> for UserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
