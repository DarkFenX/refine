use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tokio::sync::Mutex;

use crate::{
    bridge::{SolSysMgr, SolarSystem},
    util::ErrorKind,
};

use super::super::SingleErr;

pub(in super::super) enum GSsResult {
    SolSys(Arc<Mutex<SolarSystem>>),
    ErrResp(Response),
}

pub(in super::super) async fn get_guarded_ss(ss_mgr: &SolSysMgr, solsys_id: &str) -> GSsResult {
    match ss_mgr.get_sol_sys(&solsys_id).await {
        Ok(ss) => GSsResult::SolSys(ss),
        Err(e) => {
            let code = match e.kind {
                ErrorKind::SolSysNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            let resp = (code, Json(SingleErr::from(e))).into_response();
            GSsResult::ErrResp(resp)
        }
    }
}
