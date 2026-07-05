use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{err::HApiError, handlers::HSingleErr, state::HAppState};

pub(crate) async fn dev_check_sol(State(state): State<HAppState>, Path(sol_id): Path<String>) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol(br_err).into_response(),
    };
    let resp = match sol.lock().await.dev_consistency_check(&state.tpool).await {
        Ok(result) => match result {
            true => StatusCode::OK.into_response(),
            false => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from_bridge(e))).into_response(),
    };
    resp
}
