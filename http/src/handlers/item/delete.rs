use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{cmd::HItemRemoveCmd, err::HApiError, state::HAppState};

pub(crate) async fn delete_item(
    State(state): State<HAppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
    WithRejection(payload, _): WithRejection<Option<Json<HItemRemoveCmd>>, HApiError>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol_item(br_err).into_response(),
    };
    let Json(payload) = payload.unwrap_or_default();
    match sol.lock().await.remove_item(&state.tpool, &item_id, payload).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(br_err) => HApiError::from_br_path_sol_item(br_err).into_response(),
    }
}
