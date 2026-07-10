use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::HFleetInfoParams;
use crate::{cmd::HFleetChangeCmd, err::HApiError, state::HAppState};

pub(crate) async fn change_fleet(
    State(state): State<HAppState>,
    Path((sol_id, fleet_id)): Path<(String, String)>,
    WithRejection(Query(params), _): WithRejection<Query<HFleetInfoParams>, HApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<HFleetChangeCmd>, HApiError>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol_fleet(br_err).into_response(),
    };
    match sol
        .lock()
        .await
        .change_fleet(
            &state.refine.tpool,
            &fleet_id,
            payload,
            params.fleet.unwrap_or_default(),
        )
        .await
    {
        Ok(item_info) => (StatusCode::OK, Json(item_info)).into_response(),
        Err(br_err) => HApiError::from_br_path_sol_fleet(br_err).into_response(),
    }
}
