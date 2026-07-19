use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{err::ApiError, state::AppState};

pub(crate) async fn dev_check_sol(State(state): State<AppState>, Path(sol_id): Path<String>) -> impl IntoResponse {
    match internal_dev_check_sol(state, sol_id).await {
        Ok(true) => StatusCode::OK.into_response(),
        Ok(false) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_dev_check_sol(state: AppState, sol_id: String) -> Result<bool, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let passed = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .dev_consistency_check(rs::dev::DecCheckCmd)
        .await;
    Ok(passed)
}
