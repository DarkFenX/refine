use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

use crate::{err::ApiError, state::AppState};

#[derive(Deserialize)]
pub(crate) struct GetSrcItemsReqBody {
    type_ids: Vec<rs::ItemTypeId>,
}

#[serde_with::serde_as]
#[derive(Serialize)]
#[serde(transparent)]
struct GetSrcItemsRespBody {
    #[serde_as(as = "serde_with::KeyValueMap<_>")]
    items: Vec<rs::src::SrcItemInfo>,
}

pub(crate) async fn get_source_items(
    State(state): State<AppState>,
    Path(src_alias): Path<String>,
    WithRejection(Json(payload), _): WithRejection<Json<GetSrcItemsReqBody>, ApiError>,
) -> impl IntoResponse {
    match internal_get_source_items(state, src_alias, payload).await {
        Ok(resp_body) => (StatusCode::OK, Json(resp_body)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_get_source_items(
    state: AppState,
    src_alias: String,
    payload: GetSrcItemsReqBody,
) -> Result<GetSrcItemsRespBody, ApiError> {
    let src_alias =
        rs::src::SrcAlias::try_pruned(&src_alias).map_err(|err| ApiError::PathSrcParseMisc(src_alias, err))?;
    let items = state
        .get_refine()
        .get_src(Some(src_alias))
        .await?
        .get_item_infos(&payload.type_ids)
        .await;
    Ok(GetSrcItemsRespBody { items })
}
