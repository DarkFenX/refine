use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrError,
    cmd::HChangeItemCommand,
    handlers::{get_guarded_sol, item::HItemInfoParams, HGSolResult, HSingleErr},
    state::HAppState,
    util::HExecError,
};

pub(crate) async fn change_item(
    State(state): State<HAppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
    Query(params): Query<HItemInfoParams>,
    Json(payload): Json<HChangeItemCommand>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol
        .lock()
        .await
        .change_item(&item_id, payload, params.item.into())
        .await
    {
        Ok(item_info) => (StatusCode::OK, Json(item_info)).into_response(),
        Err(br_err) => {
            let code = match &br_err {
                HBrError::ItemIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(exec_err) => match exec_err {
                    HExecError::ItemNotFoundPrimary(_) => StatusCode::NOT_FOUND,
                    HExecError::NotBoosterSideEffect(_) => StatusCode::CONFLICT,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(br_err))).into_response()
        }
    };
    resp
}
