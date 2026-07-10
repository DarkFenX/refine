use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::HSolInfoParams;
use crate::{err::HApiError, state::HAppState};

pub(crate) async fn get_sol(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<HSolInfoParams>, HApiError>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol(br_err).into_response(),
    };
    match sol
        .lock()
        .await
        .get_sol(
            &state.tpool,
            params.sol.unwrap_or_default(),
            params.fleet.unwrap_or_default(),
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await
    {
        Ok(sol_info) => (StatusCode::OK, Json(sol_info)).into_response(),
        Err(br_err) => HApiError::from_br_path_sol(br_err).into_response(),
    }
}
