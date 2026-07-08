use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use super::query::HSrcInfoParams;
use crate::{err::HApiError, state::HAppState};

#[derive(Deserialize)]
pub(crate) struct HCreateSrcReq {
    data_version: String,
    data_base_url: String,
    make_default: Option<bool>,
}

pub(crate) async fn create_source(
    State(state): State<HAppState>,
    Path(src_alias): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<HSrcInfoParams>, HApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<HCreateSrcReq>, HApiError>,
) -> impl IntoResponse {
    let data_version = payload.data_version;
    let data_base_url = payload.data_base_url;
    let make_default = payload.make_default.unwrap_or(false);
    match state
        .src_mgr
        .add(
            &state.tpool,
            src_alias,
            data_version,
            data_base_url,
            make_default,
            params.src.unwrap_or_default(),
        )
        .await
    {
        Ok(src_info) => (StatusCode::CREATED, Json(src_info)).into_response(),
        Err(br_err) => HApiError::from_br_path_src(br_err).into_response(),
    }
}
