use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

use crate::{
    err::{ApiError, ApiErrorIndexed},
    state::AppState,
};

#[derive(Deserialize)]
#[serde(transparent)]
pub(crate) struct FitBatchReqBody(Vec<serde_json::Value>);

#[derive(Serialize)]
#[serde(transparent)]
struct FitBatchResp(rs::CmdResps);

pub(crate) async fn batch_fit(
    State(state): State<AppState>,
    Path((sol_id, fit_id)): Path<(String, String)>,
    WithRejection(Json(payload), _): WithRejection<Json<FitBatchReqBody>, ApiError>,
) -> impl IntoResponse {
    match internal_batch_fit(state, sol_id, fit_id, payload).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_batch_fit(
    state: AppState,
    sol_id: String,
    fit_id: String,
    payload: FitBatchReqBody,
) -> Result<FitBatchResp, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let fit_id = rs::FitId::from_str(&fit_id)?;
    let mut cmds = Vec::with_capacity(payload.0.len());
    for (index, raw_cmd) in payload.0.into_iter().enumerate() {
        match serde_json::from_value(raw_cmd) {
            Ok(cmd) => cmds.push(cmd),
            Err(de_err) => return Err(ApiError::BatchParse(ApiErrorIndexed::new(index, de_err))),
        }
    }
    let cmd_resps = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_fit(fit_id)
        .await?
        .hybrid_batch(cmds)
        .await?;
    Ok(FitBatchResp(cmd_resps))
}
