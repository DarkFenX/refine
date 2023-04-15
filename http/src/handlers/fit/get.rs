use axum::{http::StatusCode, response::IntoResponse};

pub(crate) async fn get_fit() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
