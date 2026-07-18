use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use super::query::SrcInfoParams;
use crate::{err::ApiError, state::AppState};

#[derive(Deserialize)]
pub(crate) struct CreateSrcReq {
    data_version: String,
    data_base_url: String,
    make_default: Option<bool>,
}

pub(crate) async fn create_source(
    State(state): State<AppState>,
    Path(src_alias): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<SrcInfoParams>, ApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<CreateSrcReq>, ApiError>,
) -> impl IntoResponse {
    match internal_create_source(state, src_alias, params, payload).await {
        Ok(src_info) => (StatusCode::CREATED, Json(src_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_create_source(
    state: AppState,
    src_alias: String,
    params: SrcInfoParams,
    payload: CreateSrcReq,
) -> Result<rs::src::SrcInfo, ApiError> {
    let data_version = payload.data_version;
    let data_base_url = payload.data_base_url;
    let make_default = payload.make_default.unwrap_or(false);
    let src = state
        .get_refine()
        .add_src_with_phb_http(src_alias, make_default, data_version, data_base_url)
        .await?;
    let src_mode = params.src.unwrap_or_default();
    Ok(src.get_info(src_mode).await)
}
