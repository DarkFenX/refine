use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{cmd::HBenchmarkCmd, err::HApiError, state::HAppState};

pub(crate) async fn dev_benchmark_sol(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    WithRejection(Json(payload), _): WithRejection<Json<HBenchmarkCmd>, HApiError>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol(br_err).into_response(),
    };
    match match payload {
        HBenchmarkCmd::AttrCalc(cmd) => sol.lock().await.dev_benchmark_attrs(&state.tpool, cmd).await,
        HBenchmarkCmd::Stats(cmd) => sol.lock().await.dev_benchmark_stats(&state.tpool, cmd).await,
        HBenchmarkCmd::TryFitItems(cmd) => sol.lock().await.dev_benchmark_try_fit_items(&state.tpool, cmd).await,
    } {
        Ok(_) => StatusCode::OK.into_response(),
        Err(br_err) => HApiError::from_br_path_sol(br_err).into_response(),
    }
}
