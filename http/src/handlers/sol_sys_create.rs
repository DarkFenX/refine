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
impl CreateSolSysErr {
    fn new_from_err(error: reefast::Error) -> CreateSolSysErr {
        CreateSolSysErr {
            error: format!("{}", error.msg),
        }
    }
}

pub(crate) async fn create_sol_sys(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSolSysReq>,
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
            (StatusCode::CREATED, Json(CreateSolSysResp { id })).into_response()
        }
        Err(e) if matches!(e.kind, reefast::ErrorKind::SrcNotFound) => {
            (StatusCode::UNPROCESSABLE_ENTITY, Json(CreateSolSysErr::new_from_err(e))).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CreateSolSysErr::new_from_err(e)),
        )
            .into_response(),
    }
}
