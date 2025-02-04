use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrError,
    cmd::HAddItemCommand,
    handlers::{get_guarded_sol, item::HItemInfoParams, HGSolResult, HSingleErr},
    state::HAppState,
    util::HExecError,
};

#[allow(clippy::let_and_return)]
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
    let resp = match guarded_sol
        .lock()
        .await
        .add_item(payload, params.item.unwrap_or_default())
        .await
    {
        Ok(item_info) => (StatusCode::CREATED, Json(item_info)).into_response(),
        Err(br_err) => {
            let code = match &br_err {
                HBrError::ExecFailed(exec_err) => match exec_err {
                    HExecError::SkillIdCollision(_) => StatusCode::CONFLICT,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(br_err))).into_response()
        }
    };
    resp
}
