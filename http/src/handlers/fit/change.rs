use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

use super::query::HFitInfoParams;
use crate::{cmd::HCmdResps, err::HApiError, info::HFitInfo, state::HAppState};

#[derive(Deserialize)]
pub(crate) struct HFitChangeReq {
    commands: Vec<serde_json::Value>,
}

#[derive(Serialize)]
struct HFitChangeResp {
    fit: HFitInfo,
    cmd_results: HCmdResps,
}

pub(crate) async fn change_fit(
    State(state): State<HAppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    WithRejection(Query(params), _): WithRejection<Query<HFitInfoParams>, HApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<HFitChangeReq>, HApiError>,
) -> impl IntoResponse {
    let sol = match state.sol_mgr.get_sol(&sol_id).await {
        Ok(sol) => sol,
        Err(br_err) => return HApiError::from_br_path_sol_fit(br_err).into_response(),
    };
    match sol
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
        Err(br_err) => HApiError::from_br_path_sol_fit(br_err).into_response(),
    }
}
