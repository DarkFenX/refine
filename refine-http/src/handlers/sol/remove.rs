use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{err::ApiError, state::AppState};

pub(crate) async fn remove_sol(State(state): State<AppState>, Path(sol_id): Path<String>) -> impl IntoResponse {
    match internal_remove_sol(state, sol_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_remove_sol(state: AppState, sol_id: String) -> Result<(), ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    state.get_refine().get_sol(sol_id).await?.remove().await?;
    Ok(())
}
