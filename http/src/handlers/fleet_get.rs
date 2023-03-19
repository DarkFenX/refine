use axum::{http::StatusCode, response::IntoResponse};

pub(crate) async fn get_fleet() -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
