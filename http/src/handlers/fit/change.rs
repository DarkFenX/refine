use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    cmd::{HChangeFitCommand, HCmdResp},
    handlers::{fit::HFitInfoParams, get_guarded_ss, HGSsResult, HSingleErr},
    info::HFitInfo,
    state::HAppState,
    util::HErrorKind,
};

#[derive(serde::Deserialize)]
pub(crate) struct HFitChangeReq {
    commands: Vec<HChangeFitCommand>,
}

#[derive(serde::Serialize)]
pub(crate) struct HFitChangeResp {
    fit: HFitInfo,
    cmd_results: Vec<HCmdResp>,
}
impl HFitChangeResp {
    pub(crate) fn new(fit: HFitInfo, cmd_results: Vec<HCmdResp>) -> Self {
        Self { fit, cmd_results }
    }
}

pub(crate) async fn change_fit(
    State(state): State<HAppState>,
    Path((ss_id, fit_id)): Path<(String, String)>,
    Query(params): Query<HFitInfoParams>,
    Json(payload): Json<HFitChangeReq>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        HGSsResult::Ss(ss) => ss,
        HGSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss
        .lock()
        .await
        .execute_change_fit_commands(&fit_id, payload.commands, params.fit.into(), params.item.into())
        .await
    {
        Ok((fit_info, cmd_results)) => {
            let resp = HFitChangeResp::new(fit_info, cmd_results);
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(e) => {
            let code = match e.kind {
                HErrorKind::FitIdCastFailed(_) => StatusCode::NOT_FOUND,
                HErrorKind::CoreError(rc::ErrorKind::FitNotFound(_), _) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(e))).into_response()
        }
    };
    resp
}
