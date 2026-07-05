use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use super::query::HFitInfoParams;
use crate::{cmd::HFitAddCmd, err::HApiError, state::HAppState};

pub(crate) async fn create_fit(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HFitInfoParams>,
    payload: Option<Json<HFitAddCmd>>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol(br_err).into_response(),
    };
    let Json(payload) = payload.unwrap_or_default();
    match sol
        .lock()
        .await
        .add_fit(
            &state.tpool,
            payload,
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await
    {
        Ok(fit_info) => (StatusCode::CREATED, Json(fit_info)).into_response(),
        Err(br_err) => HApiError::from_br_path_sol(br_err).into_response(),
    }
}
