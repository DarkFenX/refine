use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    cmd::{HChangeSsCommand, HCmdResp},
    handlers::{get_guarded_ss, ss::HSsInfoParams, HGSsResult, HSingleErr},
    info::HSsInfo,
    state::HAppState,
};

#[derive(serde::Deserialize)]
pub(crate) struct HSsChangeReq {
    commands: Vec<HChangeSsCommand>,
}

#[derive(serde::Serialize)]
pub(crate) struct HSsChangeResp {
    solar_system: HSsInfo,
    cmd_results: Vec<HCmdResp>,
}
impl HSsChangeResp {
    pub(crate) fn new(ss_info: HSsInfo, cmd_results: Vec<HCmdResp>) -> Self {
        Self {
            solar_system: ss_info,
            cmd_results,
        }
    }
}

pub(crate) async fn change_ss(
    State(state): State<HAppState>,
    Path(ss_id): Path<String>,
    Query(params): Query<HSsInfoParams>,
    Json(payload): Json<HSsChangeReq>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        HGSsResult::Ss(ss) => ss,
        HGSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss
        .lock()
        .await
        .execute_change_ss_commands(
            payload.commands,
            params.ss.into(),
            params.fleet.into(),
            params.fit.into(),
            params.item.into(),
        )
        .await
    {
        Ok((ss_info, cmd_results)) => {
            let resp = HSsChangeResp::new(ss_info, cmd_results);
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from(e))).into_response(),
    };
    resp
}
