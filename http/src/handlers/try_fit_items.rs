use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{cmd::HTryFitItemsCmd, err::HApiError, state::HAppState};

pub(crate) async fn try_fit_items(
    State(state): State<HAppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    WithRejection(Json(payload), _): WithRejection<Json<HTryFitItemsCmd>, HApiError>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol_fit(br_err).into_response(),
    };
    match sol.lock().await.try_fit_items(&state.tpool, &fit_id, payload).await {
        Ok(valid_type_ids) => (StatusCode::OK, Json(valid_type_ids)).into_response(),
        Err(br_err) => HApiError::from_br_path_sol_fit(br_err).into_response(),
    }
}
