use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{cmd::HBenchmarkCmd, err::HApiError, handlers::shared::HSingleErr, state::HAppState};

#[allow(clippy::let_and_return)]
pub(crate) async fn dev_benchmark_sol(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Json(payload): Json<HBenchmarkCmd>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_bridge_with_empty_path(br_err).into_response(),
    };
    let resp = match match payload {
        HBenchmarkCmd::AttrCalc(cmd) => sol.lock().await.dev_benchmark_attrs(&state.tpool, cmd).await,
        HBenchmarkCmd::Stats(cmd) => sol.lock().await.dev_benchmark_stats(&state.tpool, cmd).await,
        HBenchmarkCmd::TryFitItems(cmd) => sol.lock().await.dev_benchmark_try_fit_items(&state.tpool, cmd).await,
    } {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from_bridge(e))).into_response(),
    };
    resp
}
