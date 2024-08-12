use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    handlers::{get_guarded_sol, sol::HSolInfoParams, HGSolResult, HSingleErr},
    state::HAppState,
};

pub(crate) async fn get_sol(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HSolInfoParams>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol
        .lock()
        .await
        .get_sol(
            params.sol.into(),
            params.fleet.into(),
            params.fit.into(),
            params.item.into(),
        )
        .await
    {
        Ok(sol_info) => (StatusCode::OK, Json(sol_info)).into_response(),
        Err(br_err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from(br_err))).into_response(),
    };
    resp
}
