use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    bridge::{HBrError, HSolMgr, HSolarSystem},
    handlers::HSingleErr,
};

pub(in crate::handlers) enum HGSolResult {
    Sol(HSolarSystem),
    ErrResp(Response),
}

pub(in crate::handlers) async fn get_guarded_sol(sol_mgr: &HSolMgr, sol_id: &str) -> HGSolResult {
    match sol_mgr.get_sol(sol_id).await {
        Ok(sol) => HGSolResult::Sol(sol),
        Err(e) => {
            let code = match e {
                HBrError::SolNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            let resp = (code, Json(HSingleErr::from_bridge(e))).into_response();
            HGSolResult::ErrResp(resp)
        }
    }
}
