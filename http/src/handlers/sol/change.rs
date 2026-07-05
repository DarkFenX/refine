use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    cmd::{HCmdResps, HSolChangeCmd},
    err::HApiError,
    handlers::{HSingleErr, sol::HSolInfoParams},
    info::HSolInfo,
    state::HAppState,
};

#[derive(Deserialize)]
pub(crate) struct HSolChangeReq {
    commands: Vec<HSolChangeCmd>,
}

#[derive(Serialize)]
struct HSolChangeResp {
    solar_system: HSolInfo,
    cmd_results: HCmdResps,
}
impl HSolChangeResp {
    pub(crate) fn new(sol_info: HSolInfo, cmd_results: HCmdResps) -> Self {
        Self {
            solar_system: sol_info,
            cmd_results,
        }
    }
}

pub(crate) async fn change_sol(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HSolInfoParams>,
    Json(payload): Json<HSolChangeReq>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_bridge_with_empty_path(br_err).into_response(),
    };
    let resp = match sol
        .lock()
        .await
        .change_sol(
            &state.tpool,
            payload.commands,
            params.sol.unwrap_or_default(),
            params.fleet.unwrap_or_default(),
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await
    {
        Ok((sol_info, cmd_results)) => {
            let resp = HSolChangeResp::new(sol_info, cmd_results);
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(br_err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from_bridge(br_err))).into_response(),
    };
    resp
}
