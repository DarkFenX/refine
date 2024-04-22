use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    cmd::{HChangeSolCommand, HCmdResp},
    handlers::{get_guarded_sol, sol::HSolInfoParams, HGSolResult, HSingleErr},
    info::HSolInfo,
    state::HAppState,
};

#[derive(serde::Deserialize)]
pub(crate) struct HSolChangeReq {
    commands: Vec<HChangeSolCommand>,
}

#[derive(serde::Serialize)]
pub(crate) struct HSolChangeResp {
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
            payload.commands,
            params.sol.into(),
            params.fleet.into(),
            params.fit.into(),
            params.item.into(),
        )
        .await
    {
        Ok((sol_info, cmd_results)) => {
            let resp = HSolChangeResp::new(sol_info, cmd_results);
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from(e))).into_response(),
    };
    resp
}
