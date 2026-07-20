use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::ValInfoParams;
use crate::{err::ApiError, state::AppState};

pub(crate) async fn validate_sol(
    State(state): State<AppState>,
    Path(sol_id): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<ValInfoParams>, ApiError>,
    WithRejection(payload, _): WithRejection<Option<Json<rs::val::ValidateSolCmd>>, ApiError>,
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
    params: ValInfoParams,
    payload: rs::val::ValidateSolCmd,
) -> Result<rs::val::SolValInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let val_info = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .validate(payload, params.validation.unwrap_or_default())
        .await;
    Ok(val_info)
}
