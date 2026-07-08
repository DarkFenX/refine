use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::HItemInfoParams;
use crate::{cmd::HItemAddCmd, err::HApiError, state::HAppState};

pub(crate) async fn create_item(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<HItemInfoParams>, HApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<HItemAddCmd>, HApiError>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol(br_err).into_response(),
    };
    match sol
        .lock()
        .await
        .add_item(&state.tpool, payload, params.item.unwrap_or_default())
        .await
    {
        Ok(item_info) => (StatusCode::CREATED, Json(item_info)).into_response(),
        Err(br_err) => HApiError::from_br_path_sol(br_err).into_response(),
    }
}
