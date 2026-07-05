use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use super::query::HFleetInfoParams;
use crate::{cmd::HFleetAddCmd, err::HApiError, state::HAppState};

pub(crate) async fn create_fleet(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HFleetInfoParams>,
    payload: Option<Json<HFleetAddCmd>>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol(br_err).into_response(),
    };
    let Json(payload) = payload.unwrap_or_default();
    match sol
        .lock()
        .await
        .add_fleet(&state.tpool, payload, params.fleet.unwrap_or_default())
        .await
    {
        Ok(fleet_info) => (StatusCode::CREATED, Json(fleet_info)).into_response(),
        Err(br_err) => HApiError::from_br_path_sol(br_err).into_response(),
    }
}
