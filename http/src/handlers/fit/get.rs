use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use super::query::HFitInfoParams;
use crate::{err::HApiError, state::HAppState};

pub(crate) async fn get_fit(
    State(state): State<HAppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    Query(params): Query<HFitInfoParams>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol_fit(br_err).into_response(),
    };
    match sol
        .lock()
        .await
        .get_fit(
            &state.tpool,
            &fit_id,
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await
    {
        Ok(fit_info) => (StatusCode::OK, Json(fit_info)).into_response(),
        Err(br_err) => HApiError::from_br_path_sol_fit(br_err).into_response(),
    }
}
