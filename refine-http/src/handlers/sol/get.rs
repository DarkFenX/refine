use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::shared::SolParams;
use crate::{err::ApiError, state::AppState};

pub(crate) async fn get_sol(
    State(state): State<AppState>,
    Path(sol_id): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<SolParams>, ApiError>,
) -> impl IntoResponse {
    match internal_get_sol(state, sol_id, params).await {
        Ok(sol_info) => (StatusCode::OK, Json(sol_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_get_sol(state: AppState, sol_id: String, params: SolParams) -> Result<rs::SolInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let sol_info = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_info(params.into_cmd())
        .await;
    Ok(sol_info)
}
