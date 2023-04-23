use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{state::AppState, util::ErrorKind};

use super::super::{get_guarded_ss, GSsResult, SingleErr};

pub(crate) async fn delete_fit(
    State(state): State<Arc<AppState>>,
    Path(ssid): Path<String>,
    Path(fid): Path<String>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ssid).await {
        GSsResult::SolSys(ss) => ss,
        GSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss.lock().await.remove_fit(&fid).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            let code = match e.kind {
                ErrorKind::FitIdCastFailed(_) => StatusCode::NOT_FOUND,
                ErrorKind::CoreError(reefast::ErrorKind::FitNotFound, _) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(SingleErr::from(e))).into_response()
        }
    };
    resp
}
