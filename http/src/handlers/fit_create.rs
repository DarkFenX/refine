use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::state::AppState;

pub(crate) async fn create_fit(State(state): State<Arc<AppState>>, Path(ssid): Path<String>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
