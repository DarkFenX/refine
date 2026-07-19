use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use crate::{err::ApiError, state::AppState};

pub(crate) async fn remove_item(
    State(state): State<AppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
    WithRejection(payload, _): WithRejection<Option<Json<rs::RemoveItemCmd>>, ApiError>,
) -> impl IntoResponse {
    let Json(payload) = payload.unwrap_or_default();
    match internal_remove_item(state, sol_id, item_id, payload).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_remove_item(
    state: AppState,
    sol_id: String,
    item_id: String,
    payload: rs::RemoveItemCmd,
) -> Result<(), ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let item_id = rs::ItemId::from_str(&item_id)?;
    state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_item(item_id)
        .await?
        .remove(payload)
        .await?;
    Ok(())
}
