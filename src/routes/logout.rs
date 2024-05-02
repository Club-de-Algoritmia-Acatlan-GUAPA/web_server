use axum::response::{IntoResponse, Redirect, Response};

use crate::session::UserSession;

#[axum_macros::debug_handler]
pub async fn logout(mut session: UserSession) -> Response {
    if let Ok(Some(_)) = session.get_user_id().await {
        // Do we need to handle failed logout?, what should we do
        let _ = session.log_out().await;
    }
    Redirect::to("/login").into_response()
}
