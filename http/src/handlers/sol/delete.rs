use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{bridge::HBrError, handlers::HSingleErr, state::HAppState};

pub(crate) async fn delete_sol(State(state): State<HAppState>, Path(id): Path<String>) -> impl IntoResponse {
    match state.sol_mgr.delete_sol(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(br_err) => {
            let code = match br_err {
                HBrError::SolNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(br_err))).into_response()
        }
    }
}
