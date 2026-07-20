use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{err::ApiError, state::AppState};

pub(crate) async fn get_item_stats(
    State(state): State<AppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
    WithRejection(payload, _): WithRejection<Option<Json<rs::stats::GetItemStatsCmd>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_get_item_stats(state, sol_id, item_id, payload).await {
        Ok(item_stats) => (StatusCode::OK, Json(item_stats)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_get_item_stats(
    state: AppState,
    sol_id: String,
    item_id: String,
    payload: rs::stats::GetItemStatsCmd,
) -> Result<rs::stats::ItemStats, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let item_id = rs::ItemId::from_str(&item_id)?;
    let item_stats = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_item(item_id)
        .await?
        .get_stats(payload)
        .await;
    Ok(item_stats)
}
