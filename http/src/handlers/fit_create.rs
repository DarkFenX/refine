use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{state::AppState, util::ErrorKind};

use super::SingleErr;

#[derive(serde::Serialize)]
struct CreateFitResp {
    id: String,
}
impl CreateFitResp {
    fn new(id: String) -> Self {
        Self { id }
    }
}

pub(crate) async fn create_fit(State(state): State<Arc<AppState>>, Path(ssid): Path<String>) -> impl IntoResponse {
    let guarded_ss = match state.ss_mgr.get_sol_sys(&ssid).await {
        Ok(ss) => ss,
        Err(e) => {
            let code = match e.kind {
                ErrorKind::SolSysNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(SingleErr::from(e))).into_response();
        }
    };
    let fit_id = match guarded_ss.lock().await.add_fit().await {
        Ok(fid) => fid,
        Err(e) => {
            let code = match e.kind {
                ErrorKind::CoreError(reefast::ErrorKind::IdAllocFailed, _) => StatusCode::SERVICE_UNAVAILABLE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(e.to_string())).into_response();
        }
    };
    (StatusCode::CREATED, Json(CreateFitResp::new(fit_id))).into_response()
}
