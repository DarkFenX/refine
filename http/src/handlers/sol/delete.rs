use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{handlers::HSingleErr, state::HAppState, util::HErrorKind};

pub(crate) async fn delete_sol(State(state): State<HAppState>, Path(id): Path<String>) -> impl IntoResponse {
    let resp = match state.sol_mgr.delete_sol(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            let code = match e.kind {
                HErrorKind::SolNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(e))).into_response()
        }
    };
    resp
}
