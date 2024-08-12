use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrError,
    handlers::{sol::HSolInfoParams, HSingleErr},
    state::HAppState,
};

#[derive(serde::Deserialize)]
pub(crate) struct HCreateSolReq {
    src_alias: Option<String>,
}

pub(crate) async fn create_sol(
    State(state): State<HAppState>,
    Query(params): Query<HSolInfoParams>,
    Json(payload): Json<HCreateSolReq>,
) -> impl IntoResponse {
    let resp = match state.src_mgr.get(payload.src_alias.as_deref()).await {
        Ok(src) => {
            let sol_info = state
                .sol_mgr
                .add_sol(
                    src,
                    params.sol.into(),
                    params.fleet.into(),
                    params.fit.into(),
                    params.item.into(),
                )
                .await;
            (StatusCode::CREATED, Json(sol_info)).into_response()
        }
        Err(br_err) => {
            let code = match br_err {
                HBrError::SrcNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
                HBrError::NoDefaultSrc => StatusCode::UNPROCESSABLE_ENTITY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(br_err))).into_response()
        }
    };
    resp
}
