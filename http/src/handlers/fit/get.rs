use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    cmd::{CmdResp, FitCommand},
    handlers::{fit::FitInfoParams, get_guarded_ss, GSsResult, SingleErr},
    info::FitInfo,
    state::AppState,
    util::ErrorKind,
};

pub(crate) async fn get_fit(
    State(state): State<AppState>,
    Path((ssid, fid)): Path<(String, String)>,
    Query(params): Query<FitInfoParams>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ssid).await {
        GSsResult::SolSys(ss) => ss,
        GSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss
        .lock()
        .await
        .get_fit(&fid, params.fit.into(), params.item.into())
        .await
    {
        Ok(fit_info) => (StatusCode::OK, Json(fit_info)).into_response(),
        Err(e) => {
            let code = match e.kind {
                ErrorKind::FitIdCastFailed(_) => StatusCode::NOT_FOUND,
                ErrorKind::CoreError(reefast::ErrorKind::FitNotFound(_), _) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(SingleErr::from(e))).into_response()
        }
    };
    resp
}
