use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    handlers::{fit::HFitInfoParams, get_guarded_ss, HGSsResult, HSingleErr},
    state::HAppState,
    util::HErrorKind,
};

pub(crate) async fn create_fit(
    State(state): State<HAppState>,
    Path(ss_id): Path<String>,
    Query(params): Query<HFitInfoParams>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        HGSsResult::Ss(ss) => ss,
        HGSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss
        .lock()
        .await
        .add_fit(params.fit.into(), params.item.into())
        .await
    {
        Ok(fit_info) => (StatusCode::CREATED, Json(fit_info)).into_response(),
        Err(e) => {
            let code = match e.kind {
                HErrorKind::CoreError(rc::ErrorKind::FitIdAllocFailed, _) => StatusCode::SERVICE_UNAVAILABLE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(HSingleErr::from(e))).into_response();
        }
    };
    resp
}
