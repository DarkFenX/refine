use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    handlers::{fleet::HFleetInfoParams, get_guarded_sol, HGSolResult, HSingleErr},
    state::HAppState,
};

pub(crate) async fn create_fleet(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HFleetInfoParams>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol
        .lock()
        .await
        .add_fleet(params.fleet.unwrap_or_default())
        .await
    {
        Ok(fleet_info) => (StatusCode::CREATED, Json(fleet_info)).into_response(),
        Err(br_err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from(br_err))).into_response(),
    };
    resp
}
