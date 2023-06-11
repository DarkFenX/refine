use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    bridge::{HGuardedSs, HSsMgr},
    handlers::HSingleErr,
    util::HErrorKind,
};

pub(in crate::handlers) enum HGSsResult {
    Ss(HGuardedSs),
    ErrResp(Response),
}

pub(in crate::handlers) async fn get_guarded_ss(ss_mgr: &HSsMgr, ss_id: &str) -> HGSsResult {
    match ss_mgr.get_ss(&ss_id).await {
        Ok(ss) => HGSsResult::Ss(ss),
        Err(e) => {
            let code = match e.kind {
                HErrorKind::SsNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            let resp = (code, Json(HSingleErr::from(e))).into_response();
            HGSsResult::ErrResp(resp)
        }
    }
}
