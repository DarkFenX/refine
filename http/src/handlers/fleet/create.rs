use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    handlers::{fleet::HFleetInfoParams, get_guarded_ss, HGSsResult, HSingleErr},
    state::HAppState,
    util::HErrorKind,
};

pub(crate) async fn create_fleet(
    State(state): State<HAppState>,
    Path(ss_id): Path<String>,
    Query(params): Query<HFleetInfoParams>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        HGSsResult::Ss(ss) => ss,
        HGSsResult::ErrResp(r) => return r,
    };
    let fleet_info = match guarded_ss.lock().await.add_fleet(params.fleet.into()).await {
        Ok(fleet_info) => fleet_info,
        Err(e) => {
            let code = match e.kind {
                HErrorKind::CoreError(rc::ErrorKind::FleetIdAllocFailed, _) => StatusCode::SERVICE_UNAVAILABLE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return (code, Json(HSingleErr::from(e))).into_response();
        }
    };
    (StatusCode::CREATED, Json(fleet_info)).into_response()
}
