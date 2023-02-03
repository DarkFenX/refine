use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::state::AppState;

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
    let mkdef = payload.make_default.unwrap_or(false);
    let r = tokio_rayon::spawn_fifo(move || {
        let dh =
            Box::new(reefast::dh_impls::phobos::PhbHttpDHandler::new(data_base_url.as_str(), data_version).unwrap());
        let ch = Box::new(reefast::ch_impls::json_file::JsonFileCHandler::new(
            "/home/dfx/Workspace/eve/reefast/cache",
            alias.as_str(),
        ));
        state.src_mgr.add(alias.as_str(), dh, ch, mkdef)
    })
    .await;
    match r {
        Ok(_) => StatusCode::CREATED,
        Err(e) if matches!(e.kind, reefast::ErrorKind::SrcAlreadyExists) => StatusCode::FORBIDDEN,
        Err(e) if matches!(e.kind, reefast::ErrorKind::SrcCacheGenFailed) => StatusCode::UNPROCESSABLE_ENTITY,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
