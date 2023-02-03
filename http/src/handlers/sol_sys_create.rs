use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::state::AppState;

#[derive(serde::Deserialize)]
pub(crate) struct CreateSolSys {
    src_alias: Option<String>,
}

pub(crate) async fn create_sol_sys(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSolSys>,
) -> impl IntoResponse {
    let src_mgr = state.src_mgr.clone();
    let r = tokio_rayon::spawn_fifo(move || match payload.src_alias {
        None => reefast::SolarSystem::new(src_mgr),
        Some(a) => reefast::SolarSystem::new_with_alias(src_mgr, &a),
    })
    .await;
    match r {
        Ok(s) => {
            let id = state.sol_sys_mgr.add_sol_sys(s).await;
            StatusCode::CREATED
        }
        Err(e) if matches!(e.kind, reefast::ErrorKind::SrcNotFound) => StatusCode::UNPROCESSABLE_ENTITY,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
