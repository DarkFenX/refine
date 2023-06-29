use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    cmd::{HCmdResp, HItemCommand},
    handlers::{get_guarded_ss, item::HItemInfoParams, HGSsResult, HSingleErr},
    info::HItemInfo,
    state::HAppState,
    util::HErrorKind,
};

#[derive(serde::Deserialize)]
pub(crate) struct HItemChangeReq {
    commands: Vec<HItemCommand>,
}

#[derive(serde::Serialize)]
pub(crate) struct HItemChangeResp {
    item: HItemInfo,
    cmd_results: Vec<HCmdResp>,
}
impl HItemChangeResp {
    pub(crate) fn new(item: HItemInfo, cmd_results: Vec<HCmdResp>) -> Self {
        Self { item, cmd_results }
    }
}

pub(crate) async fn change_item(
    State(state): State<HAppState>,
    Path((ss_id, item_id)): Path<(String, String)>,
    Query(params): Query<HItemInfoParams>,
    Json(payload): Json<HItemChangeReq>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        HGSsResult::Ss(ss) => ss,
        HGSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss
        .lock()
        .await
        .execute_item_commands(&item_id, payload.commands, params.item.into())
        .await
    {
        Ok((item_info, cmd_results)) => {
            let resp = HItemChangeResp::new(item_info, cmd_results);
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(e) => {
            let code = match e.kind {
                HErrorKind::ItemIdCastFailed(_) => StatusCode::NOT_FOUND,
                HErrorKind::CoreError(rc::ErrorKind::ItemIdNotFound(_), _) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(e))).into_response()
        }
    };
    resp
}
