use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    cmd::HValidateFitCmd,
    err::{HApiError, HBrError, HExecError},
    handlers::{HSingleErr, validate::HValidInfoParams},
    state::HAppState,
};

pub(crate) async fn validate_fit(
    State(state): State<HAppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    Query(params): Query<HValidInfoParams>,
    payload: Option<Json<HValidateFitCmd>>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol(br_err).into_response(),
    };
    let Json(payload) = payload.unwrap_or_default();
    let resp = match sol
        .lock()
        .await
        .validate_fit(&state.tpool, &fit_id, payload, params.validation.unwrap_or_default())
        .await
    {
        Ok(valid_info) => (StatusCode::OK, Json(valid_info)).into_response(),
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
