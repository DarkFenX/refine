use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrError,
    cmd::HRemoveItemCmd,
    handlers::{get_guarded_sol, HGSolResult, HSingleErr},
    state::HAppState,
    util::HExecError,
};

pub(crate) async fn delete_item(
    State(state): State<HAppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
    payload: Option<Json<HRemoveItemCmd>>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let Json(payload) = payload.unwrap_or_default();
    let resp = match guarded_sol.lock().await.remove_item(&item_id, payload).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(br_err) => {
            let code = match &br_err {
                HBrError::ItemIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(exec_err) => match exec_err {
                    HExecError::ItemNotFoundPrimary(_) => StatusCode::NOT_FOUND,
                    HExecError::UnremovableAutocharge(_) => StatusCode::FORBIDDEN,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(br_err))).into_response()
        }
    };
    resp
}
