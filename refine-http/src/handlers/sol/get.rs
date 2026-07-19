use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::SolInfoParams;
use crate::{err::ApiError, state::AppState};

pub(crate) async fn get_sol(
    State(state): State<AppState>,
    Path(sol_id): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<SolInfoParams>, ApiError>,
) -> impl IntoResponse {
    match internal_get_sol(state, sol_id, params).await {
        Ok(sol_info) => (StatusCode::CREATED, Json(sol_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_get_sol(state: AppState, sol_id: String, params: SolInfoParams) -> Result<rs::SolInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let sol_info = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_info(
            params.sol.unwrap_or_default(),
            params.fleet.unwrap_or_default(),
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await;
    Ok(sol_info)
}
