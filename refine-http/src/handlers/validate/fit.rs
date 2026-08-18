use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use super::shared::ValParams;
use crate::{err::ApiError, state::AppState};

#[derive(Default, Deserialize)]
#[serde(transparent)]
pub(crate) struct FitValReqBody(rs::val::ValOptions);

pub(crate) async fn validate_fit(
    State(state): State<AppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    WithRejection(Query(params), _): WithRejection<Query<ValParams>, ApiError>,
    WithRejection(payload, _): WithRejection<Option<Json<FitValReqBody>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_validate_fit(state, sol_id, fit_id, params, payload).await {
        Ok(val_info) => (StatusCode::OK, Json(val_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_validate_fit(
    state: AppState,
    sol_id: String,
    fit_id: String,
    params: ValParams,
    payload: FitValReqBody,
) -> Result<rs::val::FitValResult, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let fit_id = rs::FitId::from_str(&fit_id)?;
    let val_info = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_fit(fit_id)
        .await?
        .validate(
            rs::val::FitValCmd::new()
                .with_options(payload.0)
                .with_info_mode(params.validation),
        )
        .await;
    Ok(val_info)
}
