use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{err::ApiError, state::AppState};

pub(crate) async fn remove_fleet(
    State(state): State<AppState>,
    Path((sol_id, fleet_id)): Path<(String, String)>,
    WithRejection(payload, _): WithRejection<Option<Json<rs::RemoveFleetCmd>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_remove_fleet(state, sol_id, fleet_id, payload).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_remove_fleet(
    state: AppState,
    sol_id: String,
    fleet_id: String,
    payload: rs::RemoveFleetCmd,
) -> Result<(), ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let fleet_id = rs::FleetId::from_str(&fleet_id)?;
    state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_fleet(fleet_id)
        .await?
        .remove(payload)
        .await;
    Ok(())
}
