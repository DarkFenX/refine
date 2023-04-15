use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{state::AppState, util::ErrorKind};

use super::super::SingleErr;

#[derive(serde::Deserialize)]
pub(crate) struct CreateSolSysReq {
    src_alias: Option<String>,
}

#[derive(serde::Serialize)]
struct CreateSolSysResp {
    id: String,
}
impl CreateSolSysResp {
    fn new(id: String) -> Self {
        Self { id }
    }
}

pub(crate) async fn create_sol_sys(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSolSysReq>,
) -> impl IntoResponse {
    let src = match state.src_mgr.get(payload.src_alias.as_deref()).await {
        Ok(s) => s,
        Err(e) => {
            let code = match e.kind {
                ErrorKind::SrcNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
                ErrorKind::NoDefaultSrc => StatusCode::UNPROCESSABLE_ENTITY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(SingleErr::from(e))).into_response();
        }
    };
    let sol_sys_id = state.ss_mgr.add_sol_sys(src).await;
    (StatusCode::CREATED, Json(CreateSolSysResp::new(sol_sys_id))).into_response()
}
