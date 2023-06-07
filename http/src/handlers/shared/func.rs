use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tokio::sync::Mutex;

use crate::{
    bridge::{HSolarSystem, SsMgr},
    handlers::SingleErr,
    util::ErrorKind,
};

pub(in crate::handlers) enum GSsResult {
    Ss(Arc<Mutex<HSolarSystem>>),
    ErrResp(Response),
}

pub(in crate::handlers) async fn get_guarded_ss(ss_mgr: &SsMgr, ss_id: &str) -> GSsResult {
    match ss_mgr.get_ss(&ss_id).await {
        Ok(ss) => GSsResult::Ss(ss),
        Err(e) => {
            let code = match e.kind {
                ErrorKind::SsNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            let resp = (code, Json(SingleErr::from(e))).into_response();
            GSsResult::ErrResp(resp)
        }
    }
}
