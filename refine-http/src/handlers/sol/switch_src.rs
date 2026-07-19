use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use super::query::SolInfoParams;
use crate::{err::ApiError, state::AppState};

#[derive(Deserialize)]
pub(crate) struct ChangeSolSrcReqBody {
    src_alias: Option<String>,
}

pub(crate) async fn switch_sol_src(
    State(state): State<AppState>,
    Path(sol_id): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<SolInfoParams>, ApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<ChangeSolSrcReqBody>, ApiError>,
) -> impl IntoResponse {
    match internal_switch_sol_src(state, sol_id, params, payload).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_switch_sol_src(
    state: AppState,
    sol_id: String,
    params: SolInfoParams,
    payload: ChangeSolSrcReqBody,
) -> Result<rs::SolInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let sol_info = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .switch_src_and_get_info(
            payload.src_alias.map(Into::into),
            params.sol.unwrap_or_default(),
            params.fleet.unwrap_or_default(),
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await?;
    Ok(sol_info)
}
