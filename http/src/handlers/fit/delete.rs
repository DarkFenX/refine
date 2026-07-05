use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{cmd::HFitRemoveCmd, err::HApiError, state::HAppState};

pub(crate) async fn delete_fit(
    State(state): State<HAppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    payload: Option<Json<HFitRemoveCmd>>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol_fit(br_err).into_response(),
    };
    let Json(payload) = payload.unwrap_or_default();
    match sol.lock().await.remove_fit(&state.tpool, &fit_id, payload).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(br_err) => HApiError::from_br_path_sol_fit(br_err).into_response(),
    }
}
