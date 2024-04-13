use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    cmd::HAddItemCommand,
    handlers::{get_guarded_ss, item::HItemInfoParams, HGSsResult, HSingleErr},
    state::HAppState,
    util::HErrorKind,
};

pub(crate) async fn create_item(
    State(state): State<HAppState>,
    Path(ss_id): Path<String>,
    Query(params): Query<HItemInfoParams>,
    Json(payload): Json<HAddItemCommand>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        HGSsResult::Ss(ss) => ss,
        HGSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss.lock().await.add_item(payload, params.item.into()).await {
        Ok(item_info) => (StatusCode::CREATED, Json(item_info)).into_response(),
        Err(e) => {
            let code = match e.kind {
                HErrorKind::CoreError(rc::ErrorKind::ItemIdAllocFailed, _) => StatusCode::SERVICE_UNAVAILABLE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(e))).into_response()
        }
    };
    resp
}
