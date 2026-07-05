use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    err::HApiError,
    handlers::{HSingleErr, sol::HSolInfoParams},
    state::HAppState,
};

#[derive(Deserialize)]
pub(crate) struct HChangeSolSrcReq {
    src_alias: Option<String>,
}

pub(crate) async fn change_sol_src(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HSolInfoParams>,
    Json(payload): Json<HChangeSolSrcReq>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_bridge_with_empty_path(br_err).into_response(),
    };
    let src = match state.src_mgr.get(payload.src_alias.as_deref()).await {
        Ok(src) => src,
        Err(br_err) => return HApiError::from_bridge_with_empty_path(br_err).into_response(),
    };
    let sol_info = match sol
        .lock()
        .await
        .change_sol_src(
            &state.tpool,
            src,
            params.sol.unwrap_or_default(),
            params.fleet.unwrap_or_default(),
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await
    {
        Ok(sol_info) => sol_info,
        Err(br_err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from_bridge(br_err))).into_response();
        }
    };
    (StatusCode::OK, Json(sol_info)).into_response()
}
