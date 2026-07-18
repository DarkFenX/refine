use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::SolInfoParams;
use crate::{err::ApiError, state::AppState};

#[derive(Default, serde::Deserialize)]
pub(crate) struct AddSolReqBody {
    src_alias: Option<String>,
    #[serde(flatten)]
    cmd: rs::AddSolCmd,
}

pub(crate) async fn add_sol(
    State(state): State<AppState>,
    WithRejection(Query(params), _): WithRejection<Query<SolInfoParams>, ApiError>,
    WithRejection(payload, _): WithRejection<Option<Json<AddSolReqBody>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_add_sol(state, params, payload).await {
        Ok(src_info) => (StatusCode::CREATED, Json(src_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_add_sol(
    state: AppState,
    params: SolInfoParams,
    payload: AddSolReqBody,
) -> Result<rs::SolInfo, ApiError> {
    let (_, sol_info) = state
        .get_refine()
        .add_sol_and_get_info(
            payload.src_alias.map(Into::into),
            payload.cmd,
            params.sol.unwrap_or_default(),
            params.fleet.unwrap_or_default(),
            params.fit.unwrap_or_default(),
            params.item.unwrap_or_default(),
        )
        .await?;
    Ok(sol_info)
}
