use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{err::ApiError, state::AppState};

pub(crate) async fn query_source(
    State(state): State<AppState>,
    Path(src_alias): Path<String>,
    WithRejection(Json(payload), _): WithRejection<Json<rs::src::SrcInfoEnumCmd>, ApiError>,
) -> impl IntoResponse {
    match internal_query_source(state, src_alias, payload).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_query_source(
    state: AppState,
    src_alias: String,
    payload: rs::src::SrcInfoEnumCmd,
) -> Result<rs::src::SrcInfoEnumResp, ApiError> {
    let src_alias = rs::SrcAlias::try_pruned(&src_alias).map_err(|err| ApiError::PathSrcParseMisc(src_alias, err))?;
    let resp = state.get_refine().get_src(Some(src_alias)).await?.query(payload).await;
    Ok(resp)
}
