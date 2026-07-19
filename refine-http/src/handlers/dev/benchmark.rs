use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{err::ApiError, state::AppState};

pub(crate) async fn dev_benchmark_sol(
    State(state): State<AppState>,
    Path(sol_id): Path<String>,
    WithRejection(Json(payload), _): WithRejection<Json<rs::dev::DevBenchmarkCmd>, ApiError>,
) -> impl IntoResponse {
    match internal_dev_benchmark_sol(state, sol_id, payload).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_dev_benchmark_sol(
    state: AppState,
    sol_id: String,
    payload: rs::dev::DevBenchmarkCmd,
) -> Result<(), ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    state.get_refine().get_sol(sol_id).await?.dev_benchmark(payload).await;
    Ok(())
}
