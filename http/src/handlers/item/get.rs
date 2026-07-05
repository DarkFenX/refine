use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{err::HApiError, handlers::item::HItemInfoParams, state::HAppState};

#[allow(clippy::let_and_return)]
pub(crate) async fn get_item(
    State(state): State<HAppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
    Query(params): Query<HItemInfoParams>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_bridge_with_empty_path(br_err).into_response(),
    };
    let resp = match sol
        .lock()
        .await
        .get_item(&state.tpool, &item_id, params.item.unwrap_or_default())
        .await
    {
        Ok(item_info) => (StatusCode::OK, Json(item_info)).into_response(),
        Err(br_err) => HApiError::from_bridge_with_item_in_path(br_err).into_response(),
    };
    resp
}
