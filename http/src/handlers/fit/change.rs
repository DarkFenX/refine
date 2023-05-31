use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    cmd::{CmdResp, FitCommand},
    handlers::{fit::FitInfoParams, get_guarded_ss, GSsResult, SingleErr},
    info::FitInfo,
    state::AppState,
    util::ErrorKind,
};

#[derive(serde::Deserialize)]
pub(crate) struct FitChangeReq {
    commands: Vec<FitCommand>,
}

#[derive(serde::Serialize)]
pub(crate) struct FitChangeResp {
    fit: FitInfo,
    cmd_results: Vec<CmdResp>,
}
impl FitChangeResp {
    pub(crate) fn new(fit: FitInfo, cmd_results: Vec<CmdResp>) -> Self {
        Self { fit, cmd_results }
    }
}

pub(crate) async fn change_fit(
    State(state): State<AppState>,
    Path((ss_id, fit_id)): Path<(String, String)>,
    Query(params): Query<FitInfoParams>,
    Json(payload): Json<FitChangeReq>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        GSsResult::Ss(ss) => ss,
        GSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss
        .lock()
        .await
        .execute_fit_commands(&fit_id, payload.commands, params.fit.into(), params.item.into())
        .await
    {
        Ok((fit_info, cmd_results)) => {
            let resp = FitChangeResp::new(fit_info, cmd_results);
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(e) => {
            let code = match e.kind {
                ErrorKind::FitIdCastFailed(_) => StatusCode::NOT_FOUND,
                ErrorKind::CoreError(rc::ErrorKind::FitNotFound(_), _) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(SingleErr::from(e))).into_response()
        }
    };
    resp
}
