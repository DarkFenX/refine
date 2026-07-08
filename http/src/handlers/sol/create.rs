use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use super::query::HSolInfoParams;
use crate::{cmd::HSolAddCmd, err::HApiError, state::HAppState};

#[derive(Default, Deserialize)]
pub(crate) struct HCreateSolReq {
    src_alias: Option<String>,
    #[serde(flatten)]
    cmd: HSolAddCmd,
}

pub(crate) async fn create_sol(
    State(state): State<HAppState>,
    WithRejection(Query(params), _): WithRejection<Query<HSolInfoParams>, HApiError>,
    WithRejection(payload, _): WithRejection<Option<Json<HCreateSolReq>>, HApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    let src = match state.src_mgr.get(payload.src_alias.as_deref()).await {
        Ok(src) => src,
        Err(br_err) => return HApiError::from_br_path_empty(br_err).into_response(),
    };
    let sol_info = state
        .sol_mgr
        .add_sol(
            &state.tpool,
            payload.cmd,
            src,
            params.sol.unwrap_or_default(),
            params.fleet.unwrap_or_default(),
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await;
    (StatusCode::CREATED, Json(sol_info)).into_response()
}
