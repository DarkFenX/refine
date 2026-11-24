use std::collections::HashMap;

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
pub(crate) struct GetSrcMutatedItemsReqBody {
    mutations: Vec<(rs::ItemTypeId, rs::ItemTypeId)>,
}

#[serde_with::serde_as]
#[derive(Serialize)]
#[serde(transparent)]
struct GetSrcMutatedItemsRespBody {
    #[serde_as(as = "serde_with::Map<_, serde_with::KeyValueMap<_>>")]
    base_types: Vec<(rs::ItemTypeId, Vec<rs::src::SrcMutatedItemInfo>)>,
}
impl GetSrcMutatedItemsRespBody {
    fn from_infos(infos: Vec<rs::src::SrcMutatedItemInfo>) -> Self {
        let mut base_types: Vec<(_, Vec<_>)> = Vec::new();
        let mut positions = HashMap::new();
        for info in infos {
            let position = *positions.entry(info.base_type_id).or_insert_with(|| {
                base_types.push((info.base_type_id, Vec::new()));
                base_types.len() - 1
            });
            base_types[position].1.push(info);
        }
        Self { base_types }
    }
}

pub(crate) async fn get_source_mutated_items(
    State(state): State<AppState>,
    Path(src_alias): Path<String>,
    WithRejection(Json(payload), _): WithRejection<Json<GetSrcMutatedItemsReqBody>, ApiError>,
) -> impl IntoResponse {
    match internal_get_source_mutated_items(state, src_alias, payload).await {
        Ok(resp_body) => (StatusCode::OK, Json(resp_body)).into_response(),
        Err(err) => err.into_response(),
    }
}

async fn internal_get_source_mutated_items(
    state: AppState,
    src_alias: String,
    payload: GetSrcMutatedItemsReqBody,
) -> Result<GetSrcMutatedItemsRespBody, ApiError> {
    let src_alias =
        rs::src::SrcAlias::try_pruned(&src_alias).map_err(|err| ApiError::PathSrcParseMisc(src_alias, err))?;
    let infos = state
        .get_refine()
        .get_src(Some(src_alias))
        .await?
        .get_mutated_item_infos(&payload.mutations)
        .await;
    Ok(GetSrcMutatedItemsRespBody::from_infos(infos))
}
