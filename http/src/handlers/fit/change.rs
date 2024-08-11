use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrError,
    cmd::{HChangeFitCommand, HCmdResp},
    handlers::{fit::HFitInfoParams, get_guarded_sol, HGSolResult, HSingleErr},
    info::HFitInfo,
    state::HAppState,
    util::HExecError,
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
    Path((sol_id, fit_id)): Path<(String, String)>,
    Query(params): Query<HFitInfoParams>,
    Json(payload): Json<HFitChangeReq>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let resp = match guarded_sol
        .lock()
        .await
        .change_fit(&fit_id, payload.commands, params.fit.into(), params.item.into())
        .await
    {
        Ok((fit_info, cmd_results)) => {
            let resp = HFitChangeResp::new(fit_info, cmd_results);
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(bridge_error) => {
            let code = match &bridge_error {
                HBrError::FitIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(exec_error) => match exec_error {
                    HExecError::FitNotFoundPrimary(_) => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(bridge_error))).into_response()
        }
    };
    resp
}
