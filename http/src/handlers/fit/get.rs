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

pub(crate) async fn get_fit(
    State(state): State<AppState>,
    Path((ss_id, fit_id)): Path<(String, String)>,
    Query(params): Query<FitInfoParams>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        GSsResult::Ss(ss) => ss,
        GSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss
        .lock()
        .await
        .get_fit(&fit_id, params.fit.into(), params.item.into())
        .await
    {
        Ok(fit_info) => (StatusCode::OK, Json(fit_info)).into_response(),
        Err(e) => {
            let code = match e.kind {
                ErrorKind::FitIdCastFailed(_) => StatusCode::NOT_FOUND,
                ErrorKind::CoreError(rc::ErrorKind::FitNotFound(_), _) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(SingleErr::from(e))).into_response()
        }
    };
    resp
}
