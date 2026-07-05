use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    cmd::HItemChangeCmd,
    err::{HApiError, HBrError, HExecError},
    handlers::{HSingleErr, item::HItemInfoParams},
    state::HAppState,
};

#[allow(clippy::let_and_return)]
pub(crate) async fn change_item(
    State(state): State<HAppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
    Query(params): Query<HItemInfoParams>,
    Json(payload): Json<HItemChangeCmd>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_bridge_with_empty_path(br_err).into_response(),
    };
    let resp = match sol
        .lock()
        .await
        .change_item(&state.tpool, &item_id, payload, params.item.unwrap_or_default())
        .await
    {
        Ok(item_info) => (StatusCode::OK, Json(item_info)).into_response(),
        Err(br_err) => {
            let code = match &br_err {
                HBrError::ItemIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(HExecError::ItemNotFoundPrimary(_)) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from_bridge(br_err))).into_response()
        }
    };
    resp
}
