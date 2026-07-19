use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::ItemInfoParams;
use crate::{err::ApiError, state::AppState};

pub(crate) async fn change_item(
    State(state): State<AppState>,
    Path((sol_id, item_id)): Path<(String, String)>,
    WithRejection(Query(params), _): WithRejection<Query<ItemInfoParams>, ApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<rs::ChangeItemEnumCmd>, ApiError>,
) -> impl IntoResponse {
    match internal_change_item(state, sol_id, item_id, params, payload).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_change_item(
    state: AppState,
    sol_id: String,
    item_id: String,
    params: ItemInfoParams,
    payload: rs::ChangeItemEnumCmd,
) -> Result<rs::ItemInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let item_id = rs::ItemId::from_str(&item_id)?;
    let (_, item_info) = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .get_item(item_id)
        .await?
        .change_and_get_info(payload, params.item.unwrap_or_default())
        .await?;
    Ok(item_info)
}
