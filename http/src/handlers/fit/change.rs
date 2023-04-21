use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{state::AppState, util::ErrorKind};

use super::super::{get_guarded_ss, GSsRes, Command};

#[derive(serde::Deserialize)]
pub(crate) struct FitChangeReq {
    commands: Vec<serde_json::Value>,
}

pub(crate) async fn change_fit(
    State(state): State<Arc<AppState>>,
    Path(ssid): Path<String>,
    Path(fid): Path<String>,
    Json(payload): Json<FitChangeReq>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ssid).await {
        GSsRes::SolSys(ss) => ss,
        GSsRes::ErrResp(r) => return r,
    };
    StatusCode::NOT_IMPLEMENTED.into_response()
}
