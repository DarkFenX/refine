use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    cmd::HItemRemoveCmd,
    err::{HApiError, HBrError, HExecError},
    handlers::HSingleErr,
    state::HAppState,
};

#[allow(clippy::let_and_return)]
pub(crate) async fn delete_item(
    State(state): State<HAppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
    payload: Option<Json<HItemRemoveCmd>>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_bridge_with_empty_path(br_err).into_response(),
    };
    let Json(payload) = payload.unwrap_or_default();
    let resp = match sol.lock().await.remove_item(&state.tpool, &item_id, payload).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(br_err) => {
            let code = match &br_err {
                HBrError::ItemIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(exec_err) => match exec_err {
                    HExecError::ItemNotFoundPrimary(_) => StatusCode::NOT_FOUND,
                    HExecError::UnremovableAutocharge => StatusCode::FORBIDDEN,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from_bridge(br_err))).into_response()
        }
    };
    resp
}
