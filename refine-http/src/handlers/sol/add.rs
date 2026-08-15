use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::shared::parse_src_alias_from_body;
use crate::{err::ApiError, state::AppState};

#[derive(Default, serde::Deserialize)]
pub(crate) struct AddSolReqBody {
    src_alias: Option<String>,
    #[serde(flatten)]
    cmd: rs::AddSolCmd,
}

pub(crate) async fn add_sol(
    State(state): State<AppState>,
    WithRejection(Query(params), _): WithRejection<Query<rs::SolInfoCmd>, ApiError>,
    WithRejection(payload, _): WithRejection<Option<Json<AddSolReqBody>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_add_sol(state, params, payload).await {
        Ok(sol_info) => (StatusCode::CREATED, Json(sol_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_add_sol(
    state: AppState,
    params: rs::SolInfoCmd,
    payload: AddSolReqBody,
) -> Result<rs::SolInfo, ApiError> {
    let src_alias = parse_src_alias_from_body(payload.src_alias)?;
    let (_, sol_info) = state
        .get_refine()
        .add_sol_and_get_info(src_alias, payload.cmd, params)
        .await?;
    Ok(sol_info)
}
