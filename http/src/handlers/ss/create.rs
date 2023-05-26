use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    handlers::{ss::SolSysInfoParams, SingleErr},
    state::AppState,
    util::ErrorKind,
};

#[derive(serde::Deserialize)]
pub(crate) struct CreateSolSysReq {
    src_alias: Option<String>,
}

pub(crate) async fn create_sol_sys(
    State(state): State<AppState>,
    Query(params): Query<SolSysInfoParams>,
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
    let ss_info = state
        .ss_mgr
        .add_sol_sys(src, params.ss.into(), params.fit.into(), params.item.into())
        .await;
    (StatusCode::CREATED, Json(ss_info)).into_response()
}
