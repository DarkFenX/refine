use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrErrorKind,
    handlers::{get_guarded_sol, HGSolResult, HSingleErr},
    state::HAppState,
    util::HExecErrorKind,
};

pub(crate) async fn delete_item(
    State(state): State<HAppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol.lock().await.remove_item(&item_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(bridge_error) => {
            let code = match &bridge_error.kind {
                HBrErrorKind::ItemIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrErrorKind::ExecFailed(exec_error) => match &exec_error.kind {
                    HExecErrorKind::CoreError(core_error) => match core_error.get_kind() {
                        rc::ErrorKind::ItemIdNotFound(_) => StatusCode::NOT_FOUND,
                        rc::ErrorKind::UnremovableItemKind(_) => StatusCode::FORBIDDEN,
                        _ => StatusCode::INTERNAL_SERVER_ERROR,
                    },
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(bridge_error))).into_response()
        }
    };
    resp
}
