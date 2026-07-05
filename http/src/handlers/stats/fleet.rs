use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    cmd::HGetFleetStatsCmd,
    err::{HApiError, HBrError, HExecError},
    handlers::HSingleErr,
    state::HAppState,
};

pub(crate) async fn get_fleet_stats(
    State(state): State<HAppState>,
    Path((sol_id, fleet_id)): Path<(String, String)>,
    payload: Option<Json<HGetFleetStatsCmd>>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_bridge_with_empty_path(br_err).into_response(),
    };
    let Json(payload) = payload.unwrap_or_default();
    let resp = match sol.lock().await.get_fleet_stats(&state.tpool, &fleet_id, payload).await {
        Ok(valid_info) => (StatusCode::OK, Json(valid_info)).into_response(),
        Err(br_err) => {
            let code = match &br_err {
                HBrError::FleetIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(HExecError::FleetNotFoundPrimary(_)) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from_bridge(br_err))).into_response()
        }
    };
    resp
}
