use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    handlers::{get_guarded_ss, HGSsResult, HSingleErr},
    state::HAppState,
};

pub(crate) async fn debug_check_ss(State(state): State<HAppState>, Path(ss_id): Path<String>) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        HGSsResult::Ss(ss) => ss,
        HGSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss.lock().await.debug_consistency_check().await {
        Ok(result) => match result {
            true => StatusCode::OK.into_response(),
            false => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from(e))).into_response(),
    };
    resp
}
