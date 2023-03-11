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
    error: String,
}

pub(crate) async fn create_sol_sys(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSolSysReq>,
) -> impl IntoResponse {
    let src = match payload.src_alias {
        Some(a) => match state.src_mgr.get(&a).await {
            Some(s) => s,
            None => {
                return (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(format!("requested source \"{a}\" not found")),
                )
                    .into_response()
            }
        },
        None => match state.src_mgr.get_default().await {
            Some(s) => s,
            None => {
                return (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json("source name is not specified, default source is not defined"),
                )
                    .into_response()
            }
        },
    };
    let sol_sys = tokio_rayon::spawn_fifo(move || reefast::SolarSystem::new(src)).await;
    let sol_sys_id = state.ss_mgr.add_sol_sys(sol_sys).await;
    (StatusCode::CREATED, Json(CreateSolSysResp { id: sol_sys_id })).into_response()
}
