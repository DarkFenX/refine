use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{err::HApiError, state::HAppState};

pub(crate) async fn delete_source(State(state): State<HAppState>, Path(src_alias): Path<String>) -> impl IntoResponse {
    match state.src_mgr.del(&src_alias).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(br_err) => HApiError::from_br_path_src(br_err).into_response(),
    }
}
