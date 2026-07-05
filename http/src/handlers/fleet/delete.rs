use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{cmd::HFleetRemoveCmd, err::HApiError, state::HAppState};

pub(crate) async fn delete_fleet(
    State(state): State<HAppState>,
    Path((sol_id, fleet_id)): Path<(String, String)>,
    payload: Option<Json<HFleetRemoveCmd>>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_bridge_with_fleet_in_path(br_err).into_response(),
    };
    let Json(payload) = payload.unwrap_or_default();
    match sol.lock().await.remove_fleet(&state.tpool, &fleet_id, payload).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(br_err) => HApiError::from_bridge_with_fleet_in_path(br_err).into_response(),
    }
}
