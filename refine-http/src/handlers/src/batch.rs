use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use crate::{
    err::{ApiError, ApiErrorIndexed},
    state::AppState,
};

#[derive(Deserialize)]
#[serde(transparent)]
pub(crate) struct SrcBatchReqBody(Vec<serde_json::Value>);

pub(crate) async fn batch_source(
    State(state): State<AppState>,
    Path(src_alias): Path<String>,
    WithRejection(Json(payload), _): WithRejection<Json<SrcBatchReqBody>, ApiError>,
) -> impl IntoResponse {
    match internal_batch_source(state, src_alias, payload).await {
        Ok(resps) => (StatusCode::OK, Json(resps)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_batch_source(
    state: AppState,
    src_alias: String,
    payload: SrcBatchReqBody,
) -> Result<Vec<rs::src::SrcInfoEnumResp>, ApiError> {
    let src_alias =
        rs::src::SrcAlias::try_pruned(&src_alias).map_err(|err| ApiError::PathSrcParseMisc(src_alias, err))?;
    let mut cmds = Vec::with_capacity(payload.0.len());
    for (index, raw_cmd) in payload.0.into_iter().enumerate() {
        match serde_json::from_value(raw_cmd) {
            Ok(cmd) => cmds.push(cmd),
            Err(de_err) => return Err(ApiError::BatchParse(ApiErrorIndexed::new(index, de_err))),
        }
    }
    let resps = state
        .get_refine()
        .get_src(Some(src_alias))
        .await?
        .query_batch(cmds)
        .await;
    Ok(resps)
}
