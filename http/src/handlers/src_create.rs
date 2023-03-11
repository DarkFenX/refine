use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{state::AppState, util::ErrorKind};

#[derive(serde::Deserialize)]
pub(crate) struct CreateSource {
    data_version: String,
    data_base_url: String,
    make_default: Option<bool>,
}

pub(crate) async fn create_source(
    State(state): State<Arc<AppState>>,
    Path(alias): Path<String>,
    Json(payload): Json<CreateSource>,
) -> impl IntoResponse {
    let data_version = payload.data_version;
    let data_base_url = payload.data_base_url;
    let make_default = payload.make_default.unwrap_or(false);
    match state
        .src_mgr
        .add(alias, data_version, data_base_url, make_default)
        .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(e) if matches!(e.kind, ErrorKind::SrcAliasNotAvailable) => StatusCode::FORBIDDEN,
        Err(e) if matches!(e.kind, ErrorKind::DhInitFailed) => StatusCode::BAD_REQUEST,
        Err(e) if matches!(e.kind, ErrorKind::SrcInitFailed) => StatusCode::UNPROCESSABLE_ENTITY,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
