use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::state::AppState;

pub(crate) async fn root() -> &'static str {
    "Welcome to REEFAST!"
}

#[derive(Deserialize)]
pub(crate) struct CreateSource {
    data_version: String,
    data_base_url: String,
    callback_base_url: String,
}

pub(crate) async fn create_source(
    State(state): State<Arc<AppState>>,
    Path(alias): Path<String>,
    Json(payload): Json<CreateSource>,
) -> impl IntoResponse {
    let data_version = payload.data_version;
    let data_base_url = payload.data_base_url;
    // let callback_base_url = payload.callback_base_url;
    let r = tokio_rayon::spawn_fifo(move || {
        let dh =
            Box::new(reefast::dh_impls::phobos::PhbHttpDHandler::new(data_base_url.as_str(), data_version).unwrap());
        let ch = Box::new(reefast::ch_impls::json_file::JsonFileCHandler::new(
            "/home/dfx/Workspace/eve/reefast/cache",
            alias.as_str(),
        ));
        state.srcmgr.add(alias.as_str(), dh, ch, false)
    })
    .await;
    match r {
        Ok(_) => StatusCode::CREATED,
        Err(e) if matches!(e.kind, reefast::ErrorKind::SrcAlreadyExists) => StatusCode::FORBIDDEN,
        Err(e) if matches!(e.kind, reefast::ErrorKind::SrcCacheGenFailed) => StatusCode::UNPROCESSABLE_ENTITY,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub(crate) async fn delete_source(State(state): State<Arc<AppState>>, Path(alias): Path<String>) -> impl IntoResponse {
    let r = tokio_rayon::spawn_fifo(move || state.srcmgr.del(alias.as_str())).await;
    match r {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) if matches!(e.kind, reefast::ErrorKind::SrcNotFound) => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
