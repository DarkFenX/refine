use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    cmd::{CmdResp, SsCommand},
    handlers::{get_guarded_ss, ss::SsInfoParams, GSsResult, SingleErr},
    info::HSsInfo,
    state::AppState,
};

#[derive(serde::Deserialize)]
pub(crate) struct SsChangeReq {
    commands: Vec<SsCommand>,
}

#[derive(serde::Serialize)]
pub(crate) struct SsChangeResp {
    solar_system: HSsInfo,
    cmd_results: Vec<CmdResp>,
}
impl SsChangeResp {
    pub(crate) fn new(ss_info: HSsInfo, cmd_results: Vec<CmdResp>) -> Self {
        Self {
            solar_system: ss_info,
            cmd_results,
        }
    }
}

pub(crate) async fn change_ss(
    State(state): State<AppState>,
    Path(ss_id): Path<String>,
    Query(params): Query<SsInfoParams>,
    Json(payload): Json<SsChangeReq>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        GSsResult::Ss(ss) => ss,
        GSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss
        .lock()
        .await
        .execute_ss_commands(
            payload.commands,
            params.ss.into(),
            params.fit.into(),
            params.item.into(),
        )
        .await
    {
        Ok((ss_info, cmd_results)) => {
            let resp = SsChangeResp::new(ss_info, cmd_results);
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(SingleErr::from(e))).into_response(),
    };
    resp
}
