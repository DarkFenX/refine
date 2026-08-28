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
pub(crate) struct SolValReqBody {
    #[serde(default)]
    options: rs::val::ValOptions,
    #[serde(default)]
    fit_ids: Vec<rs::FitId>,
}

pub(crate) async fn validate_sol(
    State(state): State<AppState>,
    Path(sol_id): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<ValParams>, ApiError>,
    WithRejection(payload, _): WithRejection<Option<Json<SolValReqBody>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_validate_sol(state, sol_id, params, payload).await {
        Ok(val_info) => (StatusCode::OK, Json(val_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_validate_sol(
    state: AppState,
    sol_id: String,
    params: ValParams,
    payload: SolValReqBody,
) -> Result<rs::val::SolValResult, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let val_info = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .validate(
            rs::val::SolValCmd::new()
                .with_options(payload.options)
                .with_fit_ids(payload.fit_ids)
                .with_info_mode(params.validation),
        )
        .await;
    Ok(val_info)
}
