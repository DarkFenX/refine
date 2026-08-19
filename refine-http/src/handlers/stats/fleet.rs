use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{err::ApiError, state::AppState};

pub(crate) async fn get_fleet_stats(
    State(state): State<AppState>,
    Path((sol_id, fleet_id)): Path<(String, String)>,
    WithRejection(payload, _): WithRejection<Option<Json<rs::stats::FleetStatsOptions>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_get_fleet_stats(state, sol_id, fleet_id, payload).await {
        Ok(fleet_stats) => (StatusCode::OK, Json(fleet_stats)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_get_fleet_stats(
    state: AppState,
    sol_id: String,
    fleet_id: String,
    payload: rs::stats::FleetStatsOptions,
) -> Result<rs::stats::FleetStatsResult, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let fleet_id = rs::FleetId::from_str(&fleet_id)?;
    let fleet_stats = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_fleet(fleet_id)
        .await?
        .get_stats(payload)
        .await;
    Ok(fleet_stats)
}
