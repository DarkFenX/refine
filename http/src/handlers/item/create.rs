use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{
    cmd::HItemAddCmd,
    err::HApiError,
    handlers::{HGSolResult, get_guarded_sol, item::HItemInfoParams},
    state::HAppState,
};

#[allow(clippy::let_and_return)]
pub(crate) async fn create_item(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HItemInfoParams>,
    WithRejection(Json(payload), _): WithRejection<Json<HItemAddCmd>, HApiError>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol
        .lock()
        .await
        .add_item(&state.tpool, payload, params.item.unwrap_or_default())
        .await
    {
        Ok(item_info) => (StatusCode::CREATED, Json(item_info)).into_response(),
        Err(br_err) => HApiError::from_bridge_with_empty_path(br_err).into_response(),
    };
    resp
}
