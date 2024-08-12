use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{bridge::HBrError, handlers::HSingleErr, state::HAppState};

#[derive(serde::Deserialize)]
pub(crate) struct HCreateSrcReq {
    data_version: String,
    data_base_url: String,
    make_default: Option<bool>,
}

pub(crate) async fn create_source(
    State(state): State<HAppState>,
    Path(alias): Path<String>,
    Json(payload): Json<HCreateSrcReq>,
) -> impl IntoResponse {
    let data_version = payload.data_version;
    let data_base_url = payload.data_base_url;
    let make_default = payload.make_default.unwrap_or(false);
    let resp = match state
        .src_mgr
        .add(alias, data_version, data_base_url, make_default)
        .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(br_err) => {
            let code = match br_err {
                HBrError::SrcAliasNotAvailable(_) => StatusCode::FORBIDDEN,
                HBrError::EdhInitFailed(_) => StatusCode::BAD_REQUEST,
                HBrError::SrcInitFailed(_) => StatusCode::UNPROCESSABLE_ENTITY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(br_err))).into_response()
        }
    };
    resp
}
