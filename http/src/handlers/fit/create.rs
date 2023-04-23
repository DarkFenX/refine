use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{state::AppState, util::ErrorKind};

use super::super::{get_guarded_ss, GSsResult, SingleErr};

pub(crate) async fn create_fit(State(state): State<Arc<AppState>>, Path(ssid): Path<String>) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ssid).await {
        GSsResult::SolSys(ss) => ss,
        GSsResult::ErrResp(r) => return r,
    };
    let fit_info = match guarded_ss.lock().await.add_fit().await {
        Ok(fit_info) => fit_info,
        Err(e) => {
            let code = match e.kind {
                ErrorKind::CoreError(reefast::ErrorKind::IdAllocFailed, _) => StatusCode::SERVICE_UNAVAILABLE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(SingleErr::from(e))).into_response();
        }
    };
    (StatusCode::CREATED, Json(fit_info)).into_response()
}
