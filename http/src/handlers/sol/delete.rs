use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{err::HApiError, state::HAppState};

pub(crate) async fn delete_sol(State(state): State<HAppState>, Path(sol_id): Path<String>) -> impl IntoResponse {
    match state.sol_mgr.delete_sol(&sol_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(br_err) => HApiError::from_br_path_sol(br_err).into_response(),
    }
}
