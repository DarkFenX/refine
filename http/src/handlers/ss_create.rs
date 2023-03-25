use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::state::AppState;

#[derive(serde::Deserialize)]
pub(crate) struct CreateSolSysReq {
    src_alias: Option<String>,
}

#[derive(serde::Serialize)]
pub(crate) struct CreateSolSysResp {
    id: String,
}

#[derive(serde::Serialize)]
pub(crate) struct CreateSolSysErr {
    code: String,
    message: String,
}

pub(crate) async fn create_sol_sys(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSolSysReq>,
) -> impl IntoResponse {
    let src = match state.src_mgr.get(payload.src_alias.as_deref()).await {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(CreateSolSysErr {
                    code: e.get_code(),
                    message: e.to_string(),
                }),
            )
                .into_response()
        }
    };
    let sol_sys = tokio_rayon::spawn_fifo(move || reefast::SolarSystem::new(src)).await;
    let sol_sys_id = state.ss_mgr.add_sol_sys(sol_sys).await;
    (StatusCode::CREATED, Json(CreateSolSysResp { id: sol_sys_id })).into_response()
}
