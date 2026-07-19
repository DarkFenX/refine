use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

use super::query::SolInfoParams;
use crate::{err::ApiError, state::AppState};

#[derive(Deserialize)]
pub(crate) struct SolChangeReqBody {
    commands: Vec<serde_json::Value>,
}

#[derive(Serialize)]
struct HSolChangeResp {
    solar_system: rs::SolInfo,
    cmd_results: rs::CmdResps,
}

pub(crate) async fn change_sol(
    State(state): State<AppState>,
    Path(sol_id): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<SolInfoParams>, ApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<SolChangeReqBody>, ApiError>,
) -> impl IntoResponse {
    match internal_change_sol(state, sol_id, params, payload).await {
        Ok(resp) => (StatusCode::CREATED, Json(resp)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_change_sol(
    state: AppState,
    sol_id: String,
    params: SolInfoParams,
    payload: SolChangeReqBody,
) -> Result<HSolChangeResp, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    //let cmds = payload.commands.into_iter().map(|raw_cmd| serde_json::from_value(raw_cmd))
    let (cmd_resps, sol_info) = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .change_and_get_info(
            Vec::new(),
            params.sol.unwrap_or_default(),
            params.fleet.unwrap_or_default(),
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await?;
    Ok(HSolChangeResp {
        solar_system: sol_info,
        cmd_results: cmd_resps,
    })
}
