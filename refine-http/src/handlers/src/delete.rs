use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{err::ApiError, state::AppState};

pub(crate) async fn delete_source(State(state): State<AppState>, Path(src_alias): Path<String>) -> impl IntoResponse {
    match internal_delete_source(state, src_alias).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_delete_source(state: AppState, src_alias: String) -> Result<(), ApiError> {
    state
        .get_refine()
        .get_src(Some(src_alias.into()))
        .await?
        .remove()
        .await?;
    Ok(())
}
