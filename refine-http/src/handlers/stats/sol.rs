use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{err::ApiError, state::AppState};

pub(crate) async fn get_sol_stats(
    State(state): State<AppState>,
    Path(sol_id): Path<String>,
    WithRejection(payload, _): WithRejection<Option<Json<rs::stats::SolStatsCmd>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_get_sol_stats(state, sol_id, payload).await {
        Ok(sol_stats) => (StatusCode::OK, Json(sol_stats)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_get_sol_stats(
    state: AppState,
    sol_id: String,
    payload: rs::stats::SolStatsCmd,
) -> Result<rs::stats::SolStatsResp, ApiError> {
    let sol_id = rs::SolId::from_str(&sol_id)?;
    let sol_stats = state.get_refine().get_sol(sol_id).await?.get_stats(payload).await;
    Ok(sol_stats)
}
