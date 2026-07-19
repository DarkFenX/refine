use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

use super::query::FitInfoParams;
use crate::{err::ApiError, state::AppState};

#[derive(Deserialize)]
pub(crate) struct FitChangeReqBody {
    commands: Vec<serde_json::Value>,
}

#[derive(Serialize)]
struct FitChangeResp {
    fit: rs::FitInfo,
    cmd_results: rs::CmdResps,
}

pub(crate) async fn change_fit(
    State(state): State<AppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    WithRejection(Query(params), _): WithRejection<Query<FitInfoParams>, ApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<FitChangeReqBody>, ApiError>,
) -> impl IntoResponse {
    match internal_change_fit(state, sol_id, fit_id, params, payload).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_change_fit(
    state: AppState,
    sol_id: String,
    fit_id: String,
    params: FitInfoParams,
    payload: FitChangeReqBody,
) -> Result<FitChangeResp, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let fit_id = rs::FitId::from_str(&fit_id)?;
    let mut cmds = Vec::with_capacity(payload.commands.len());
    for (index, raw_cmd) in payload.commands.into_iter().enumerate() {
        match serde_json::from_value(raw_cmd) {
            Ok(cmd) => cmds.push(cmd),
            Err(de_err) => return Err(ApiError::BatchParseFailed(index, de_err.to_string())),
        }
    }
    let (cmd_resps, fit_info) = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_fit(fit_id)
        .await?
        .change_and_get_info(cmds, params.fit.unwrap_or_default(), params.item.unwrap_or_default())
        .await?;
    Ok(FitChangeResp {
        fit: fit_info,
        cmd_results: cmd_resps,
    })
}
