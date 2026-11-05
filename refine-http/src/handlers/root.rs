use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

use crate::{err::ApiError, state::AppState};

#[derive(Deserialize)]
pub(crate) struct RootParams {
    #[serde(default)]
    src: rs::src::SrcInfoMode,
    #[serde(default)]
    sol: rs::SolInfoMode,
    #[serde(default)]
    fleet: rs::FleetInfoMode,
    #[serde(default)]
    fit: rs::FitInfoMode,
    #[serde(default)]
    item: rs::ItemInfoMode,
}
impl RootParams {
    fn into_cmd(self) -> rs::RefineInfoCmd {
        rs::RefineInfoCmd::new()
            .with_src(self.src)
            .with_sol(self.sol)
            .with_fleet(self.fleet)
            .with_fit(self.fit)
            .with_item(self.item)
    }
}

#[derive(Serialize)]
struct RootInfo {
    threads_async: usize,
    threads_sync_standard: usize,
    threads_sync_heavy: usize,
    srcs: Vec<rs::src::SrcInfo>,
    sols: Vec<rs::SolInfo>,
}

pub(crate) async fn root(
    State(state): State<AppState>,
    WithRejection(Query(params), _): WithRejection<Query<RootParams>, ApiError>,
) -> impl IntoResponse {
    let refine_info = state.get_refine().get_info(params.into_cmd()).await;
    let root_info = RootInfo {
        threads_async: tokio::runtime::Handle::current().metrics().num_workers(),
        threads_sync_standard: refine_info.threads_standard,
        threads_sync_heavy: refine_info.threads_heavy,
        srcs: refine_info.srcs,
        sols: refine_info.sols,
    };
    (StatusCode::OK, Json(root_info))
}
