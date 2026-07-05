use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::HItemInfoParams;
use crate::{cmd::HItemChangeCmd, err::HApiError, state::HAppState};

pub(crate) async fn change_item(
    State(state): State<HAppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
    Query(params): Query<HItemInfoParams>,
    WithRejection(Json(payload), _): WithRejection<Json<HItemChangeCmd>, HApiError>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_bridge_with_item_in_path(br_err).into_response(),
    };
    match sol
        .lock()
        .await
        .change_item(&state.tpool, &item_id, payload, params.item.unwrap_or_default())
        .await
    {
        Ok(item_info) => (StatusCode::OK, Json(item_info)).into_response(),
        Err(br_err) => HApiError::from_bridge_with_item_in_path(br_err).into_response(),
    }
}
