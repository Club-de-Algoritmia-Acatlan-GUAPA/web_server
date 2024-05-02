use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use axum::http::StatusCode;
use base64::{engine::general_purpose, Engine as _};
use http::header::{HeaderMap, CONTENT_TYPE};
use rand::{thread_rng, RngCore};

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

pub fn parse_boundary(headers: &HeaderMap) -> Option<String> {
    let content_type = headers.get(CONTENT_TYPE)?.to_str().ok()?;
    multer::parse_boundary(content_type).ok()
}

fn generate_cookie(len: usize) -> String {
    let mut key = vec![0u8; len];
    thread_rng().fill_bytes(&mut key);
    general_purpose::STANDARD.encode(key)
}

pub fn id_from_cookie_value(string: &str) -> Result<String, base64::DecodeError> {
    let decoded = general_purpose::STANDARD.decode(string)?;
    let hash = blake3::hash(&decoded);
    Ok(base64::encode(&hash.as_bytes()))
}
