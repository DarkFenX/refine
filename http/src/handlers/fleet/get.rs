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

pub(crate) async fn get_fleet(
    State(state): State<HAppState>,
    Path((ss_id, fleet_id)): Path<(String, String)>,
    Query(params): Query<HFleetInfoParams>,
) -> impl IntoResponse {
    let guarded_ss = match get_guarded_ss(&state.ss_mgr, &ss_id).await {
        HGSsResult::Ss(ss) => ss,
        HGSsResult::ErrResp(r) => return r,
    };
    let resp = match guarded_ss.lock().await.get_fleet(&fleet_id, params.fleet.into()).await {
        Ok(fleet_info) => (StatusCode::OK, Json(fleet_info)).into_response(),
        Err(e) => {
            let code = match e.kind {
                HErrorKind::FleetIdCastFailed(_) => StatusCode::NOT_FOUND,
                HErrorKind::CoreError(rc::ErrorKind::FleetNotFound(_), _) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (code, Json(HSingleErr::from(e))).into_response()
        }
    };
    resp
}
