use axum::response::{IntoResponse, Redirect, Response};

use crate::session::UserSession;

#[axum_macros::debug_handler]
pub async fn logout(mut session: UserSession) -> Response {
    if session.get_user_id().is_some() {
        session.log_out();
    }
    Redirect::to("/login").into_response()
}
