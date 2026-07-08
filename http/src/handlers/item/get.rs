use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::HItemInfoParams;
use crate::{err::HApiError, state::HAppState};

pub(crate) async fn get_item(
    State(state): State<HAppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
    WithRejection(Query(params), _): WithRejection<Query<HItemInfoParams>, HApiError>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol_item(br_err).into_response(),
    };
    match sol
        .lock()
        .await
        .get_item(&state.tpool, &item_id, params.item.unwrap_or_default())
        .await
    {
        Ok(item_info) => (StatusCode::OK, Json(item_info)).into_response(),
        Err(br_err) => HApiError::from_br_path_sol_item(br_err).into_response(),
    }
}
