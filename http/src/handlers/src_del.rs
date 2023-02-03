use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::state::AppState;

pub(crate) async fn delete_source(State(state): State<Arc<AppState>>, Path(alias): Path<String>) -> impl IntoResponse {
    let r = tokio_rayon::spawn_fifo(move || state.src_mgr.del(alias.as_str())).await;
    match r {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) if matches!(e.kind, reefast::ErrorKind::SrcNotFound) => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
