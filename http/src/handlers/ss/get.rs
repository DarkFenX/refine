use axum::{http::StatusCode, response::IntoResponse};

pub(crate) async fn get_sol_sys() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
