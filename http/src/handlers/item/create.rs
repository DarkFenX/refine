use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrErrorKind,
    cmd::HAddItemCommand,
    handlers::{get_guarded_sol, item::HItemInfoParams, HGSolResult, HSingleErr},
    state::HAppState,
    util::HExecErrorKind,
};

pub(crate) async fn create_item(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HItemInfoParams>,
    Json(payload): Json<HAddItemCommand>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol.lock().await.add_item(payload, params.item.into()).await {
        Ok(item_info) => (StatusCode::CREATED, Json(item_info)).into_response(),
        Err(bridge_error) => {
            let code = match &bridge_error.kind {
                HBrErrorKind::ExecFailed(exec_error) => match &exec_error.kind {
                    HExecErrorKind::CoreError(core_error) => match core_error.get_kind() {
                        rc::ErrorKind::ItemIdAllocFailed => StatusCode::SERVICE_UNAVAILABLE,
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
