use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrError,
    handlers::{get_guarded_sol, sol::HSolInfoParams, HGSolResult, HSingleErr},
    state::HAppState,
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
        Err(br_err) => {
            let code = match &br_err {
                HBrError::SrcNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
                HBrError::NoDefaultSrc => StatusCode::UNPROCESSABLE_ENTITY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(HSingleErr::from(br_err))).into_response();
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
        Err(br_err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(HSingleErr::from(br_err))).into_response();
        }
    };
    (StatusCode::OK, Json(sol_info)).into_response()
}
