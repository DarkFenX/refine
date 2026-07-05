use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    cmd::HTryFitItemsCmd,
    err::{HApiError, HBrError, HExecError},
    handlers::HSingleErr,
    state::HAppState,
};

pub(crate) async fn try_fit_items(
    State(state): State<HAppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    Json(payload): Json<HTryFitItemsCmd>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_bridge_with_empty_path(br_err).into_response(),
    };
    let resp = match sol.lock().await.try_fit_items(&state.tpool, &fit_id, payload).await {
        Ok(valid_type_ids) => (StatusCode::OK, Json(valid_type_ids)).into_response(),
        Err(br_err) => {
            let code = match &br_err {
                HBrError::FitIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(HExecError::FitNotFoundPrimary(_)) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from_bridge(br_err))).into_response()
        }
    };
    resp
}
