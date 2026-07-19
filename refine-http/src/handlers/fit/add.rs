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

pub(crate) async fn add_fit(
    State(state): State<AppState>,
    Path(sol_id): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<FitInfoParams>, ApiError>,
    WithRejection(payload, _): WithRejection<Option<Json<rs::AddFitCmd>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_add_fit(state, sol_id, params, payload).await {
        Ok(fit_info) => (StatusCode::CREATED, Json(fit_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_add_fit(
    state: AppState,
    sol_id: String,
    params: FitInfoParams,
    payload: rs::AddFitCmd,
) -> Result<rs::FitInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let (_, fit_info) = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .add_fit_and_get_info(payload, params.fit.unwrap_or_default(), params.item.unwrap_or_default())
        .await?;
    Ok(fit_info)
}
