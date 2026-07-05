use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{cmd::HGetFitStatsCmd, err::HApiError, state::HAppState};

pub(crate) async fn get_fit_stats(
    State(state): State<HAppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    payload: Option<Json<HGetFitStatsCmd>>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol_fit(br_err).into_response(),
    };
    let Json(payload) = payload.unwrap_or_default();
    match sol.lock().await.get_fit_stats(&state.tpool, &fit_id, payload).await {
        Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
        Err(br_err) => HApiError::from_br_path_sol_fit(br_err).into_response(),
    }
}
