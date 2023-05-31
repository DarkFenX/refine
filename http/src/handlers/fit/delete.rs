use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    handlers::{get_guarded_ss, GSsResult, SingleErr},
    state::AppState,
    util::ErrorKind,
};

pub(crate) async fn delete_fit(
    State(state): State<AppState>,
    Path((ss_id, fit_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        GSsResult::Ss(ss) => ss,
        GSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss.lock().await.remove_fit(&fit_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
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
