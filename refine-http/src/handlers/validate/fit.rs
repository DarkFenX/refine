use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::HValidInfoParams;
use crate::{err::ApiError, state::AppState};

pub(crate) async fn validate_fit(
    State(state): State<AppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    WithRejection(Query(params), _): WithRejection<Query<HValidInfoParams>, ApiError>,
    WithRejection(payload, _): WithRejection<Option<Json<rs::val::ValidateFitCmd>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_validate_fit(state, sol_id, fit_id, params, payload).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_validate_fit(
    state: AppState,
    sol_id: String,
    fit_id: String,
    params: HValidInfoParams,
    payload: rs::val::ValidateFitCmd,
) -> Result<rs::val::FitValInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let fit_id = rs::FitId::from_str(&fit_id)?;
    let val_info = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_fit(fit_id)
        .await?
        .validate(payload, params.validation.unwrap_or_default())
        .await;
    Ok(val_info)
}
