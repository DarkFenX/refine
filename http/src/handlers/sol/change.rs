use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    cmd::{HChangeSolCommand, HCmdResp},
    handlers::{HGSolResult, HSingleErr, get_guarded_sol, sol::HSolInfoParams},
    info::HSolInfo,
    state::HAppState,
};

#[derive(Deserialize)]
pub(crate) struct HSolChangeReq {
    commands: Vec<HChangeSolCommand>,
}

#[derive(Serialize)]
struct HSolChangeResp {
    solar_system: HSolInfo,
    cmd_results: Vec<HCmdResp>,
}
impl HSolChangeResp {
    pub(crate) fn new(sol_info: HSolInfo, cmd_results: Vec<HCmdResp>) -> Self {
        Self {
            solar_system: sol_info,
            cmd_results,
        }
    }
}

#[allow(clippy::let_and_return)]
pub(crate) async fn change_sol(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HSolInfoParams>,
    Json(payload): Json<HSolChangeReq>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol
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
