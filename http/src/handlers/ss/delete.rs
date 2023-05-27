use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{handlers::SingleErr, state::AppState, util::ErrorKind};

pub(crate) async fn delete_ss(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match state.ss_mgr.delete_ss(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            let code = match e.kind {
                ErrorKind::SsNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(SingleErr::from(e))).into_response()
        }
    }
}
