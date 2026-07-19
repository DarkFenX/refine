use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::FleetInfoParams;
use crate::{err::ApiError, state::AppState};

pub(crate) async fn get_fleet(
    State(state): State<AppState>,
    Path((sol_id, fleet_id)): Path<(String, String)>,
    WithRejection(Query(params), _): WithRejection<Query<FleetInfoParams>, ApiError>,
) -> impl IntoResponse {
    match internal_get_fleet(state, sol_id, fleet_id, params).await {
        Ok(fleet_info) => (StatusCode::OK, Json(fleet_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_get_fleet(
    state: AppState,
    sol_id: String,
    fleet_id: String,
    params: FleetInfoParams,
) -> Result<rs::FleetInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let fleet_id = rs::FleetId::from_str(&fleet_id)?;
    let fleet_info = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_fleet(fleet_id)
        .await?
        .get_info(params.fleet.unwrap_or_default())
        .await;
    Ok(fleet_info)
}
