use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use axum::http::StatusCode;

pub async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", err),
    )
}

pub fn get_current_timestamp() -> Result<u64> {
    let start = SystemTime::now();
    Ok(start.duration_since(UNIX_EPOCH)?.as_millis().try_into()?)
}
