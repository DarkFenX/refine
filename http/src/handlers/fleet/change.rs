use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrErrorKind,
    cmd::HChangeFleetCmd,
    handlers::{fleet::HFleetInfoParams, get_guarded_sol, HGSolResult, HSingleErr},
    state::HAppState,
    util::HExecErrorKind,
};

pub(crate) async fn change_fleet(
    State(state): State<HAppState>,
    Path((sol_id, fleet_id)): Path<(String, String)>,
    Query(params): Query<HFleetInfoParams>,
    Json(payload): Json<HChangeFleetCmd>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol
        .lock()
        .await
        .change_fleet(&fleet_id, payload, params.fleet.into())
        .await
    {
        Ok(item_info) => (StatusCode::OK, Json(item_info)).into_response(),
        Err(bridge_error) => {
            let code = match &bridge_error.kind {
                HBrErrorKind::FleetIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrErrorKind::ExecFailed(exec_error) => match &exec_error.kind {
                    HExecErrorKind::CoreError(core_error) => match core_error.get_kind() {
                        rc::ErrorKind::FleetNotFound(_) => StatusCode::NOT_FOUND,
                        _ => StatusCode::INTERNAL_SERVER_ERROR,
                    },
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(bridge_error))).into_response()
        }
    };
    resp
}
