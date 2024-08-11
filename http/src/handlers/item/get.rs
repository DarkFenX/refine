use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrError,
    handlers::{get_guarded_sol, item::HItemInfoParams, HGSolResult, HSingleErr},
    state::HAppState,
    util::HExecError,
};

pub(crate) async fn get_item(
    State(state): State<HAppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
    Query(params): Query<HItemInfoParams>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol.lock().await.get_item(&item_id, params.item.into()).await {
        Ok(item_info) => (StatusCode::OK, Json(item_info)).into_response(),
        Err(bridge_error) => {
            let code = match &bridge_error {
                HBrError::ItemIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(exec_error) => match exec_error {
                    HExecError::ItemNotFoundPrimary(_) => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(bridge_error))).into_response()
        }
    };
    resp
}
