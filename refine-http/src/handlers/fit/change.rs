use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{err::ApiError, state::AppState};

pub(crate) async fn change_fit(
    State(state): State<AppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    WithRejection(Query(params), _): WithRejection<Query<rs::FitInfoCmdBr>, ApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<rs::FitChangeEnumCmd>, ApiError>,
) -> impl IntoResponse {
    match internal_change_fit(state, sol_id, fit_id, params, payload).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_change_fit(
    state: AppState,
    sol_id: String,
    fit_id: String,
    params: rs::FitInfoCmdBr,
    payload: rs::FitChangeEnumCmd,
) -> Result<rs::FitInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let fit_id = rs::FitId::from_str(&fit_id)?;
    let (_, fit_info) = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_fit(fit_id)
        .await?
        .change_and_get_info(payload, params)
        .await?;
    Ok(fit_info)
}
