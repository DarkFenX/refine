use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{err::ApiError, state::AppState};

#[derive(serde::Deserialize)]
pub(crate) struct AddSrcReqBody {
    data_version: String,
    data_format: String,
    data_base_url: String,
    make_default: Option<bool>,
}

pub(crate) async fn add_source(
    State(state): State<AppState>,
    Path(src_alias): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<rs::src::SrcInfoArgs>, ApiError>,
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
    params: rs::src::SrcInfoArgs,
    payload: AddSrcReqBody,
) -> Result<rs::src::SrcInfo, ApiError> {
    let src_alias =
        rs::src::SrcAlias::try_pruned(&src_alias).map_err(|err| ApiError::PathSrcParseOnAdd(src_alias, err))?;
    let ed_base_url = payload.data_base_url;
    let ed_version = payload.data_version;
    let make_default = payload.make_default.unwrap_or(false);
    let ed_handler = match payload.data_format.to_lowercase().as_str() {
        "phb" | "phobos" => redh::PhbHttpEdh::try_new(ed_base_url, ed_version)
            .map_err(|err| ApiError::EdhInit(Box::new(err)))?
            .into(),
        "sde" => redh::SdeHttpEdh::try_new(ed_base_url, ed_version)
            .map_err(|err| ApiError::EdhInit(Box::new(err)))?
            .into(),
        _ => return Err(ApiError::EdhNotFound(payload.data_format)),
    };
    let ad_cacher = state
        .get_cache_dir()
        .map(|cache_dir| radc::PostcardZfsAdc::new(cache_dir, src_alias).into());
    let src = state
        .get_refine()
        .add_src(src_alias, make_default, ed_handler, ad_cacher)
        .await?;
    Ok(src.get_info(params).await)
}
