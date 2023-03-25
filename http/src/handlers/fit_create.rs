use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::state::AppState;

#[derive(serde::Serialize)]
pub(crate) struct CreateFitResp {
    id: String,
}

#[derive(serde::Serialize)]
pub(crate) struct CreateFitErr {
    error: String,
}

pub(crate) async fn create_fit(State(state): State<Arc<AppState>>, Path(ssid): Path<String>) -> impl IntoResponse {
    let guarded_ss = match state.ss_mgr.get_sol_sys(&ssid).await {
        Some(ss) => ss,
        None => return (StatusCode::NOT_FOUND, Json(format!("solar system not found"))).into_response(),
    };
    let fit_id = match guarded_ss.lock().await.add_fit().await {
        Ok(fid) => fid,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    };
    (StatusCode::CREATED, Json(CreateFitResp { id: fit_id.to_string() })).into_response()
}
