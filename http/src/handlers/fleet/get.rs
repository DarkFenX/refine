use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    err::{HApiError, HBrError, HExecError},
    handlers::{HSingleErr, fleet::HFleetInfoParams},
    state::HAppState,
};

#[allow(clippy::let_and_return)]
pub(crate) async fn get_fleet(
    State(state): State<HAppState>,
    Path((sol_id, fleet_id)): Path<(String, String)>,
    Query(params): Query<HFleetInfoParams>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_bridge_with_empty_path(br_err).into_response(),
    };
    let resp = match sol
        .lock()
        .await
        .get_fleet(&state.tpool, &fleet_id, params.fleet.unwrap_or_default())
        .await
    {
        Ok(fleet_info) => (StatusCode::OK, Json(fleet_info)).into_response(),
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
