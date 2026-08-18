use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{err::ApiError, state::AppState};

pub(crate) async fn try_fit_items(
    State(state): State<AppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    WithRejection(Json(payload), _): WithRejection<Json<rs::trial::FitTryItemsCmd>, ApiError>,
) -> impl IntoResponse {
    match internal_try_fit_items(state, sol_id, fit_id, payload).await {
        Ok(fittable) => (StatusCode::OK, Json(fittable)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_try_fit_items(
    state: AppState,
    sol_id: String,
    fit_id: String,
    payload: rs::trial::FitTryItemsCmd,
) -> Result<Vec<rs::ItemTypeId>, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let fit_id = rs::FitId::from_str(&fit_id)?;
    let fittable = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_fit(fit_id)
        .await?
        .try_items(payload)
        .await;
    Ok(fittable)
}
