use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    cmd::HBenchmarkCmd,
    handlers::{HGSolResult, get_guarded_sol, shared::HSingleErr},
    state::HAppState,
};

#[allow(clippy::let_and_return)]
pub(crate) async fn dev_benchmark_sol(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Json(payload): Json<HBenchmarkCmd>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match match payload {
        HBenchmarkCmd::AttrCalc(cmd) => guarded_sol.lock().await.dev_benchmark_attrs(cmd).await,
        HBenchmarkCmd::TryFitItems(cmd) => guarded_sol.lock().await.dev_benchmark_try_fit_items(cmd).await,
    } {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from(e))).into_response(),
    };
    resp
}
