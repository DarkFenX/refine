use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    cmd::{HCmdResps, HFitChangeCmd},
    err::{HApiError, HBrError, HExecError},
    handlers::{HSingleErr, fit::HFitInfoParams},
    info::HFitInfo,
    state::HAppState,
};

#[derive(Deserialize)]
pub(crate) struct HFitChangeReq {
    commands: Vec<HFitChangeCmd>,
}

#[derive(Serialize)]
struct HFitChangeResp {
    fit: HFitInfo,
    cmd_results: HCmdResps,
}

pub(crate) async fn change_fit(
    State(state): State<HAppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    Query(params): Query<HFitInfoParams>,
    Json(payload): Json<HFitChangeReq>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_bridge_with_empty_path(br_err).into_response(),
    };
    let resp = match sol
        .lock()
        .await
        .change_fit(
            &state.tpool,
            &fit_id,
            payload.commands,
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await
    {
        Ok((fit_info, cmd_results)) => {
            let resp = HFitChangeResp {
                fit: fit_info,
                cmd_results,
            };
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(br_err) => {
            let code = match &br_err {
                HBrError::FitIdCastFailed(_) => StatusCode::NOT_FOUND,
                HBrError::ExecFailed(HExecError::FitNotFoundPrimary(_)) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from_bridge(br_err))).into_response()
        }
    };
    resp
}
