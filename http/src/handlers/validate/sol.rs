use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use super::query::HValidInfoParams;
use crate::{cmd::HValidateSolCmd, err::HApiError, state::HAppState};

pub(crate) async fn validate_sol(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HValidInfoParams>,
    payload: Option<Json<HValidateSolCmd>>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol(br_err).into_response(),
    };
    let Json(payload) = payload.unwrap_or_default();
    match sol
        .lock()
        .await
        .validate_sol(&state.tpool, payload, params.validation.unwrap_or_default())
        .await
    {
        Ok(valid_info) => (StatusCode::OK, Json(valid_info)).into_response(),
        Err(br_err) => HApiError::from_br_path_sol(br_err).into_response(),
    }
}
