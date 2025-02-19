use axum::{extract::Path, response::IntoResponse};
#[axum_macros::debug_handler]
pub async fn htmx_redirect(Path(redirect): Path<String>) -> impl IntoResponse {
    ([("HX-Location", format!("/{}", redirect))], "Redirecting").into_response()
}
