use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;

use super::query::HItemInfoParams;
use crate::{err::ApiError, state::AppState};

pub(crate) async fn add_item(
    State(state): State<AppState>,
    Path(sol_id): Path<String>,
    WithRejection(Query(params), _): WithRejection<Query<HItemInfoParams>, ApiError>,
    WithRejection(Json(payload), _): WithRejection<Json<rs::AddItemEnumCmd>, ApiError>,
) -> impl IntoResponse {
    match internal_add_item(state, sol_id, params, payload).await {
        Ok(fit_info) => (StatusCode::CREATED, Json(fit_info)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_add_item(
    state: AppState,
    sol_id: String,
    params: HItemInfoParams,
    payload: rs::AddItemEnumCmd,
) -> Result<rs::ItemInfo, ApiError> {
    let sol_id = rs::SolarSystemId::from_str(&sol_id)?;
    let (_, item_info) = state
        .get_refine()
        .get_sol(sol_id)
        .await?
        .add_item_and_get_info(payload, params.item.unwrap_or_default())
        .await?;
    Ok(item_info)
}
