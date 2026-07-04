use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use super::query::HSrcInfoParams;
use crate::{bridge::HBrError, handlers::HSingleErr, state::HAppState};

#[derive(Deserialize)]
pub(crate) struct HCreateSrcReq {
    data_version: String,
    data_base_url: String,
    make_default: Option<bool>,
}

pub(crate) async fn create_source(
    State(state): State<HAppState>,
    Path(alias): Path<String>,
    Query(params): Query<HSrcInfoParams>,
    Json(payload): Json<HCreateSrcReq>,
) -> impl IntoResponse {
    let data_version = payload.data_version;
    let data_base_url = payload.data_base_url;
    let make_default = payload.make_default.unwrap_or(false);
    match state
        .src_mgr
        .add(
            &state.tpool,
            alias,
            data_version,
            data_base_url,
            make_default,
            params.src.unwrap_or_default(),
        )
        .await
    {
        Ok(src_info) => (StatusCode::CREATED, Json(src_info)).into_response(),
        Err(br_err) => {
            let code = match br_err {
                HBrError::SrcAliasNotAvailable(_) => StatusCode::FORBIDDEN,
                HBrError::EdhInitFailed(_) => StatusCode::BAD_REQUEST,
                HBrError::SrcInitFailed(_) => StatusCode::UNPROCESSABLE_ENTITY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from_bridge(br_err))).into_response()
        }
    }
}
