use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::shared::SrcInfoParams;
use crate::{err::ApiError, state::AppState};

pub(crate) async fn get_source(
    State(state): State<AppState>,
    Path(src_alias): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<SrcInfoParams>, ApiError>,
) -> impl IntoResponse {
    match internal_get_source(state, src_alias, params).await {
        Ok(..) => StatusCode::OK.into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_get_source(
    state: AppState,
    src_alias: String,
    params: SrcInfoParams,
) -> Result<rs::src::SrcInfo, ApiError> {
    let src_alias = rs::src::SrcAlias::try_pruned(&src_alias).map_err(ApiError::PathSrcParseFailedMisc)?;
    let src_info = state
        .get_refine()
        .get_src(Some(src_alias))
        .await?
        .get_info(params.src.unwrap_or_default())
        .await;
    Ok(src_info)
}
