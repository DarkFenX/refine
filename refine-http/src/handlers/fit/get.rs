use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::FitInfoParams;
use crate::{err::ApiError, state::AppState};

pub(crate) async fn get_fit(
    State(state): State<AppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    WithRejection(Query(params), _): WithRejection<Query<FitInfoParams>, ApiError>,
) -> impl IntoResponse {
    match internal_get_fit(state, sol_id, fit_id, params).await {
        Ok(sol_info) => (StatusCode::CREATED, Json(sol_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_get_fit(
    state: AppState,
    sol_id: String,
    fit_id: String,
    params: FitInfoParams,
) -> Result<rs::FitInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let fit_id = rs::FitId::from_str(&fit_id)?;
    let fit_info = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_fit(fit_id)
        .await?
        .get_info(params.fit.unwrap_or_default(), params.item.unwrap_or_default())
        .await;
    Ok(fit_info)
}
