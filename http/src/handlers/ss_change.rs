use axum::{http::StatusCode, response::IntoResponse};

#[derive(serde::Deserialize)]
pub(crate) struct ChangeSolSysReq {}

#[derive(serde::Serialize)]
pub(crate) struct ChangeSolSysResp {}

pub(crate) async fn change_sol_sys() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}
