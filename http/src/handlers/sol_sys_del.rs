use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::state::AppState;

pub(crate) async fn delete_sol_sys(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    match state.sol_sys_mgr.delete_sol_sys(&id).await {
        true => StatusCode::NO_CONTENT,
        false => StatusCode::NOT_FOUND,
    }
}
