use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use tokio::sync::oneshot;

use crate::{glue, state::AppState};

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
    let nstate = state.clone();
    let r = tokio_rayon::spawn_fifo(|| glue::create_source(nstate, alias, data_version, data_base_url)).await;
    match rx.await.unwrap() {
        glue::TaskStatus::Success => StatusCode::CREATED,
        glue::TaskStatus::Error => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
