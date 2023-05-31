use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    handlers::{fit::FitInfoParams, get_guarded_ss, GSsResult, SingleErr},
    state::AppState,
    util::ErrorKind,
};

pub(crate) async fn create_fit(
    State(state): State<AppState>,
    Path(ss_id): Path<String>,
    Query(params): Query<FitInfoParams>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        GSsResult::Ss(ss) => ss,
        GSsResult::ErrResp(r) => return r,
    };
    let fit_info = match guarded_ss
        .lock()
        .await
        .add_fit(params.fit.into(), params.item.into())
        .await
    {
        Ok(fit_info) => fit_info,
        Err(e) => {
            let code = match e.kind {
                ErrorKind::CoreError(reefast_core::ErrorKind::FitIdAllocFailed, _) => StatusCode::SERVICE_UNAVAILABLE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(SingleErr::from(e))).into_response();
        }
    };
    (StatusCode::CREATED, Json(fit_info)).into_response()
}
