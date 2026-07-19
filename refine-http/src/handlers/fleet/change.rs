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

pub(crate) async fn change_fleet(
    State(state): State<AppState>,
    Path((sol_id, fleet_id)): Path<(String, String)>,
    WithRejection(Query(params), _): WithRejection<Query<FleetInfoParams>, ApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<rs::ChangeFleetCmd>, ApiError>,
) -> impl IntoResponse {
    match internal_change_fleet(state, sol_id, fleet_id, params, payload).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_change_fleet(
    state: AppState,
    sol_id: String,
    fleet_id: String,
    params: FleetInfoParams,
    payload: rs::ChangeFleetCmd,
) -> Result<rs::FleetInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let fleet_id = rs::FleetId::from_str(&fleet_id)?;
    let fleet_info = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_fleet(fleet_id)
        .await?
        .change_and_get_info(payload, params.fleet.unwrap_or_default())
        .await?;
    Ok(fleet_info)
}
