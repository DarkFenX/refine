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

pub(crate) async fn add_fleet(
    State(state): State<AppState>,
    Path(sol_id): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<FleetInfoParams>, ApiError>,
    WithRejection(payload, _): WithRejection<Option<Json<rs::AddFleetCmd>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_add_fleet(state, sol_id, params, payload).await {
        Ok(fleet_info) => (StatusCode::CREATED, Json(fleet_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_add_fleet(
    state: AppState,
    sol_id: String,
    params: FleetInfoParams,
    payload: rs::AddFleetCmd,
) -> Result<rs::FleetInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let (_, fleet_info) = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .add_fleet_and_get_info(payload, params.fleet.unwrap_or_default())
        .await?;
    Ok(fleet_info)
}
