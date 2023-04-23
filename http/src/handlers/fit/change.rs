use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{bridge::FitCommand, state::AppState, util::ErrorKind};

use super::super::{get_guarded_ss, GSsResult, SingleErr};

#[derive(serde::Deserialize)]
pub(crate) struct FitChangeReq {
    commands: Vec<FitCommand>,
}

pub(crate) async fn change_fit(
    State(state): State<Arc<AppState>>,
    Path((ssid, fid)): Path<(String, String)>,
    Json(payload): Json<FitChangeReq>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ssid).await {
        GSsResult::SolSys(ss) => ss,
        GSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss
        .lock()
        .await
        .execute_fit_commands(&fid, &payload.commands)
        .await
    {
        Ok(cmd_resps) => (StatusCode::OK, Json(cmd_resps)).into_response(),
        Err(e) => {
            let code = StatusCode::INTERNAL_SERVER_ERROR;
            (code, Json(SingleErr::from(e))).into_response()
        }
    };
    resp
}
