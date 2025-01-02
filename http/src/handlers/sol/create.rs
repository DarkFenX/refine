use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    bridge::HBrError,
    cmd::HAddSolCmd,
    handlers::{sol::HSolInfoParams, HSingleErr},
    state::HAppState,
    util::{body_json_or_empty::JsonOrEmpty, HExecError},
};

#[derive(serde::Deserialize)]
pub(crate) struct HCreateSolReq {
    src_alias: Option<String>,
    #[serde(flatten)]
    cmd: HAddSolCmd,
}
impl Default for HCreateSolReq {
    fn default() -> Self {
        Self {
            src_alias: None,
            cmd: Default::default(),
        }
    }
}

pub(crate) async fn create_sol(
    State(state): State<HAppState>,
    Query(params): Query<HSolInfoParams>,
    JsonOrEmpty(payload): JsonOrEmpty<HCreateSolReq>,
) -> impl IntoResponse {
    let src = match state.src_mgr.get(payload.src_alias.as_deref()).await {
        Ok(src) => src,
        Err(br_err) => {
            let code = match br_err {
                HBrError::SrcNotFound(_) => StatusCode::UNPROCESSABLE_ENTITY,
                HBrError::NoDefaultSrc => StatusCode::UNPROCESSABLE_ENTITY,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(HSingleErr::from(br_err))).into_response();
        }
    };
    let sol_info = match state
        .sol_mgr
        .add_sol(
            payload.cmd,
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
            let code = match &br_err {
                HBrError::ExecFailed(exec_err) => match &exec_err {
                    HExecError::InvalidDmgProfileEm(_)
                    | HExecError::InvalidDmgProfileTherm(_)
                    | HExecError::InvalidDmgProfileKin(_)
                    | HExecError::InvalidDmgProfileExpl(_)
                    | HExecError::InvalidDmgProfileTotal(_) => StatusCode::BAD_REQUEST,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(HSingleErr::from(br_err))).into_response();
        }
    };
    (StatusCode::CREATED, Json(sol_info)).into_response()
}
