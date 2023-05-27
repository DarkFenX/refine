use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    handlers::{get_guarded_ss, ss::SsInfoParams, GSsResult, SingleErr},
    state::AppState,
};

pub(crate) async fn get_ss(
    State(state): State<AppState>,
    Path(ss_id): Path<String>,
    Query(params): Query<SsInfoParams>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        GSsResult::Ss(ss) => ss,
        GSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss
        .lock()
        .await
        .get_info(params.ss.into(), params.fit.into(), params.item.into())
        .await
    {
        Ok(ss_info) => (StatusCode::OK, Json(ss_info)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(SingleErr::from(e))).into_response(),
    };
    resp
}
