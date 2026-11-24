use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use crate::{err::ApiError, state::AppState};

#[derive(Deserialize)]
pub(crate) struct GetSrcMutatedItemsReqBody {
    mutations: Vec<(rs::ItemTypeId, rs::ItemTypeId)>,
}

pub(crate) async fn get_source_mutated_items(
    State(state): State<AppState>,
    Path(src_alias): Path<String>,
    WithRejection(Json(payload), _): WithRejection<Json<GetSrcMutatedItemsReqBody>, ApiError>,
) -> impl IntoResponse {
    match internal_get_source_mutated_items(state, src_alias, payload).await {
        Ok(item_infos) => (StatusCode::OK, Json(item_infos)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_get_source_mutated_items(
    state: AppState,
    src_alias: String,
    payload: GetSrcMutatedItemsReqBody,
) -> Result<rs::src::SrcMutatedItemInfos, ApiError> {
    let src_alias =
        rs::src::SrcAlias::try_pruned(&src_alias).map_err(|err| ApiError::PathSrcParseMisc(src_alias, err))?;
    let item_infos = state
        .get_refine()
        .get_src(Some(src_alias))
        .await?
        .get_mutated_item_infos(&payload.mutations)
        .await;
    Ok(item_infos)
}
