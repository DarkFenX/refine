use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    bridge::HBrError,
    cmd::HGetFleetStatsCmd,
    handlers::{HGSolResult, HSingleErr, get_guarded_sol},
    state::HAppState,
    util::HExecError,
};

#[allow(clippy::let_and_return)]
pub(crate) async fn get_fleet_stats(
    State(state): State<HAppState>,
    Path((sol_id, fleet_id)): Path<(String, String)>,
    payload: Option<Json<HGetFleetStatsCmd>>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let Json(payload) = payload.unwrap_or_default();
    let resp = match guarded_sol
        .lock()
        .await
        .get_fleet_stats(&state.tpool, &fleet_id, payload)
        .await
    {
        Ok(valid_info) => (StatusCode::OK, Json(valid_info)).into_response(),
        Err(br_err) => {
            let code = match &br_err {
                HBrError::FleetIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(HExecError::FleetNotFoundPrimary(_)) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(br_err))).into_response()
        }
    };
    resp
}
