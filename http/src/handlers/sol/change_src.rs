use crate::{
    handlers::{get_guarded_sol, sol::HSolInfoParams, HGSolResult, HSingleErr},
    state::HAppState,
    util::HErrorKind,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeSolSrcReq {
    src_alias: Option<String>,
}

pub(crate) async fn change_sol_src(
    State(state): State<HAppState>,
    Path(sol_id): Path<String>,
    Query(params): Query<HSolInfoParams>,
    Json(payload): Json<HChangeSolSrcReq>,
) -> impl IntoResponse {
    let guarded_sol = match get_guarded_sol(&state.sol_mgr, &sol_id).await {
        HGSolResult::Sol(sol) => sol,
        HGSolResult::ErrResp(r) => return r,
    };
    let src = match state.src_mgr.get(payload.src_alias.as_deref()).await {
        Ok(src) => src,
        Err(e) => {
            let code = match e.kind {
                HErrorKind::SrcNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
                HErrorKind::NoDefaultSrc => StatusCode::UNPROCESSABLE_ENTITY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(HSingleErr::from(e))).into_response();
        }
    };
    let sol_info = match guarded_sol
        .lock()
        .await
        .change_sol_src(
            src,
            params.sol.into(),
            params.fleet.into(),
            params.fit.into(),
            params.item.into(),
        )
        .await
    {
        Ok(sol_info) => sol_info,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from(e))).into_response(),
    };
    (StatusCode::OK, Json(sol_info)).into_response()
}
