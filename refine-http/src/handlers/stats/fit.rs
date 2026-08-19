use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{err::ApiError, state::AppState};

pub(crate) async fn get_fit_stats(
    State(state): State<AppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    WithRejection(payload, _): WithRejection<Option<Json<rs::stats::FitStatsOptions>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_get_fit_stats(state, sol_id, fit_id, payload).await {
        Ok(fit_stats) => (StatusCode::OK, Json(fit_stats)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_get_fit_stats(
    state: AppState,
    sol_id: String,
    fit_id: String,
    payload: rs::stats::FitStatsOptions,
) -> Result<rs::stats::FitStatsResult, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let fit_id = rs::FitId::from_str(&fit_id)?;
    let fit_stats = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_fit(fit_id)
        .await?
        .get_stats(payload)
        .await;
    Ok(fit_stats)
}
