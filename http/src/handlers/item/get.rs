use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    handlers::{get_guarded_ss, item::ItemInfoParams, GSsResult, SingleErr},
    state::AppState,
    util::ErrorKind,
};

pub(crate) async fn get_item(
    State(state): State<AppState>,
    Path((ss_id, item_id)): Path<(String, String)>,
    Query(params): Query<ItemInfoParams>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        GSsResult::Ss(ss) => ss,
        GSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss.lock().await.get_item(&item_id, params.item.into()).await {
        Ok(fit_info) => (StatusCode::OK, Json(fit_info)).into_response(),
        Err(e) => {
            let code = match e.kind {
                ErrorKind::ItemIdCastFailed(_) => StatusCode::NOT_FOUND,
                ErrorKind::CoreError(reefast::ErrorKind::ItemIdNotFound(_), _) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(SingleErr::from(e))).into_response()
        }
    };
    resp
}
