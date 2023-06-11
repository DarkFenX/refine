use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    handlers::{ss::HSsInfoParams, HSingleErr},
    state::HAppState,
    util::HErrorKind,
};

#[derive(serde::Deserialize)]
pub(crate) struct HCreateSsReq {
    src_alias: Option<String>,
}

pub(crate) async fn create_ss(
    State(state): State<HAppState>,
    Query(params): Query<HSsInfoParams>,
    Json(payload): Json<HCreateSsReq>,
) -> impl IntoResponse {
    let src = match state.src_mgr.get(payload.src_alias.as_deref()).await {
        Ok(s) => s,
        Err(e) => {
            let code = match e.kind {
                HErrorKind::SrcNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
                HErrorKind::NoDefaultSrc => StatusCode::UNPROCESSABLE_ENTITY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(HSingleErr::from(e))).into_response();
        }
    };
    let ss_info = state
        .ss_mgr
        .add_ss(src, params.ss.into(), params.fit.into(), params.item.into())
        .await;
    (StatusCode::CREATED, Json(ss_info)).into_response()
}
