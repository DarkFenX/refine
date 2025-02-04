use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrError,
    handlers::{fleet::HFleetInfoParams, get_guarded_sol, HGSolResult, HSingleErr},
    state::HAppState,
    util::HExecError,
};

#[allow(clippy::let_and_return)]
pub(crate) async fn get_fleet(
    State(state): State<HAppState>,
    Path((sol_id, fleet_id)): Path<(String, String)>,
    Query(params): Query<HFleetInfoParams>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol
        .lock()
        .await
        .get_fleet(&fleet_id, params.fleet.unwrap_or_default())
        .await
    {
        Ok(fleet_info) => (StatusCode::OK, Json(fleet_info)).into_response(),
        Err(br_err) => {
            let code = match &br_err {
                HBrError::FleetIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(exec_err) => match &exec_err {
                    HExecError::FleetNotFoundPrimary(_) => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(br_err))).into_response()
        }
    };
    resp
}
