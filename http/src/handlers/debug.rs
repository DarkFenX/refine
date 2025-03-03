use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    handlers::{HGSolResult, HSingleErr, get_guarded_sol},
    state::HAppState,
};

#[allow(clippy::let_and_return)]
pub(crate) async fn debug_check_sol(State(state): State<HAppState>, Path(sol_id): Path<String>) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol.lock().await.debug_consistency_check().await {
        Ok(result) => match result {
            true => StatusCode::OK.into_response(),
            false => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from(e))).into_response(),
    };
    resp
}
