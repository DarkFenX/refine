use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::shared::SrcInfoParams;
use crate::{err::ApiError, state::AppState};

#[derive(serde::Deserialize)]
pub(crate) struct AddSrcReqBody {
    data_version: String,
    data_base_url: String,
    make_default: Option<bool>,
}

pub(crate) async fn add_source(
    State(state): State<AppState>,
    Path(src_alias): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<SrcInfoParams>, ApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<AddSrcReqBody>, ApiError>,
) -> impl IntoResponse {
    match internal_add_source(state, src_alias, params, payload).await {
        Ok(src_info) => (StatusCode::CREATED, Json(src_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_add_source(
    state: AppState,
    src_alias: String,
    params: SrcInfoParams,
    payload: AddSrcReqBody,
) -> Result<rs::src::SrcInfo, ApiError> {
    let src_alias =
        rs::src::SrcAlias::try_pruned(&src_alias).map_err(|err| ApiError::PathSrcParseOnAdd(src_alias, err))?;
    let ed_base_url = payload.data_base_url;
    let ed_version = payload.data_version;
    let make_default = payload.make_default.unwrap_or(false);
    let ed_handler = redh::PhbHttpEdh::try_new(ed_base_url, ed_version)
        .map_err(|err| ApiError::EdhInit(Box::new(err)))?
        .into();
    let ad_cacher = state
        .get_cache_dir()
        .map(|cache_dir| radc::PostcardZfsAdc::new(cache_dir, src_alias).into());
    let src = state
        .get_refine()
        .add_src(src_alias, make_default, ed_handler, ad_cacher)
        .await?;
    let src_mode = params.src.unwrap_or_default();
    Ok(src.get_info(src_mode).await)
}
