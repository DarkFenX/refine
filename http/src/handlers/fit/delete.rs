use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    bridge::HBrError,
    handlers::{HGSolResult, HSingleErr, get_guarded_sol},
    state::HAppState,
    util::HExecError,
};

#[allow(clippy::let_and_return)]
pub(crate) async fn delete_fit(
    State(state): State<HAppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol.lock().await.remove_fit(&fit_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(br_err) => {
            let code = match &br_err {
                HBrError::FitIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(HExecError::FitNotFoundPrimary(_)) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(br_err))).into_response()
        }
    };
    resp
}
