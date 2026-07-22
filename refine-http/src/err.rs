use axum::{
    Json,
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub(crate) enum ApiError {
    #[error("{}", .0.body_text())]
    Query(QueryRejection),
    #[error("{}", .0.body_text())]
    Json(JsonRejection),
    #[error("{1}")]
    BatchParseFailed(usize, String),
    #[error("{1}")]
    BackrefRenderFailed(usize, #[source] rs::err::BackrefRenderError),
    // Source-related
    #[error("{0}")]
    PathSrcNotFound(#[from] rs::src::err::GetSrcError),
    #[error("{0}")]
    SrcAddFailed(#[from] rs::src::err::AddSrcError),
    #[error("{0}")]
    SrcRemoveFailed(#[from] rs::src::err::RemoveSrcError),
    // Solar system-related
    #[error("{0}")]
    PathSolParseFailed(#[from] rs::err::ParseSolarSystemIdError),
    #[error("{0}")]
    PathSolNotFound(#[from] rs::err::GetSolError),
    #[error("{0}")]
    SolAddFailed(#[from] rs::err::AddSolError),
    #[error("{0}")]
    SolChangeFailed(usize, #[source] rs::err::ChangeSolEnumError),
    #[error("{0}")]
    SolRemoveFailed(#[from] rs::err::RemoveSolError),
    #[error("{0}")]
    SolSrcSwitch(#[from] rs::err::SolSwitchSrcError),
    // Fleet-related
    #[error("{0}")]
    PathFleetParseFailed(#[from] rs::err::ParseFleetIdError),
    #[error("{0}")]
    PathFleetNotFound(#[from] rs::err::GetFleetError),
    #[error("{0}")]
    FleetAddFailed(#[from] rs::err::AddFleetError),
    #[error("{0}")]
    FleetChangeFailed(#[from] rs::err::ChangeFleetError),
    // Fit-related
    #[error("{0}")]
    PathFitParseFailed(#[from] rs::err::ParseFitIdError),
    #[error("{0}")]
    PathFitNotFound(#[from] rs::err::GetFitError),
    #[error("{0}")]
    FitAddFailed(#[from] rs::err::AddFitError),
    #[error("{0}")]
    FitChangeFailed(usize, #[source] rs::err::ChangeFitEnumError),
    // Item-related
    #[error("{0}")]
    PathItemParseFailed(#[from] rs::err::ParseItemIdError),
    #[error("{0}")]
    PathItemNotFound(#[from] rs::err::GetItemError),
    #[error("{0}")]
    ItemAddFailed(#[from] rs::err::AddItemEnumError),
    #[error("{0}")]
    ItemChangeFailed(#[from] rs::err::ChangeItemEnumError),
    #[error("{0}")]
    ItemRemoveFailed(#[from] rs::err::RemoveItemError),
}

#[derive(Serialize)]
struct ApiErrorResponse {
    code: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmd_index: Option<usize>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Codes & messages
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ApiError {
    fn get_codes(&self) -> (StatusCode, &'static str) {
        match self {
            Self::Query(_) => (StatusCode::BAD_REQUEST, "PRM-001"),
            Self::Json(_) => (StatusCode::BAD_REQUEST, "JSN-001"),
            Self::BatchParseFailed(_, _) => (StatusCode::BAD_REQUEST, "JSN-002"),
            Self::BackrefRenderFailed(_, _) => (StatusCode::BAD_REQUEST, "BRF-001"),
            ////////////////////////////////////////////////////////////////////////////////////////
            // Source-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathSrcNotFound(rs_err) => match rs_err {
                rs::src::err::GetSrcError::SrcNotFound(_) => (StatusCode::NOT_FOUND, "SRC-001"),
                rs::src::err::GetSrcError::DefaultNotDefined => (StatusCode::NOT_FOUND, "SRC-002"),
            },
            Self::SrcAddFailed(rs_err) => match rs_err {
                rs::src::err::AddSrcError::SrcAliasNotAvailable(_) => (StatusCode::FORBIDDEN, "SRC-003"),
                rs::src::err::AddSrcError::EdhInitFailed(_) => (StatusCode::BAD_REQUEST, "EDH-001"),
                rs::src::err::AddSrcError::SrcInitFailed(_) => (StatusCode::UNPROCESSABLE_ENTITY, "SNT-001"),
            },
            Self::SrcRemoveFailed(rs_err) => match rs_err {
                rs::src::err::RemoveSrcError::SrcNotFound(_) => (StatusCode::NOT_FOUND, "SRC-004"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Solar system-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathSolParseFailed(_) => (StatusCode::NOT_FOUND, "SOL-001"),
            Self::PathSolNotFound(rs_err) => match rs_err {
                rs::err::GetSolError::SolNotFound(_) => (StatusCode::NOT_FOUND, "SOL-002"),
            },
            Self::SolAddFailed(rs_err) => match rs_err {
                rs::err::AddSolError::GetSrcFailed(_) => (StatusCode::BAD_REQUEST, "SOL-003"),
            },
            // TODO: adjust error codes based on specific responses
            Self::SolChangeFailed(_, _) => (StatusCode::BAD_REQUEST, "SOL-000"),
            Self::SolRemoveFailed(rs_err) => match rs_err {
                rs::err::RemoveSolError::SolNotFound(_) => (StatusCode::NOT_FOUND, "SOL-004"),
            },
            Self::SolSrcSwitch(rs_err) => match rs_err {
                rs::err::SolSwitchSrcError::SrcGetFailed(_) => (StatusCode::BAD_REQUEST, "SOL-005"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Fleet-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathFleetParseFailed(_) => (StatusCode::NOT_FOUND, "FLT-001"),
            Self::PathFleetNotFound(_) => (StatusCode::NOT_FOUND, "FLT-002"),
            Self::FleetAddFailed(rs_err) => match rs_err {
                rs::err::AddFleetError::FitAddFailed(_) => (StatusCode::BAD_REQUEST, "FLT-003"),
            },
            Self::FleetChangeFailed(_) => (StatusCode::BAD_REQUEST, "FLT-004"),
            ////////////////////////////////////////////////////////////////////////////////////////
            // Fit-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathFitParseFailed(_) => (StatusCode::NOT_FOUND, "FIT-001"),
            Self::PathFitNotFound(_) => (StatusCode::NOT_FOUND, "FIT-002"),
            Self::FitAddFailed(rs_err) => match rs_err {
                rs::err::AddFitError::FleetSetFailed(_) => (StatusCode::BAD_REQUEST, "FIT-003"),
            },
            // TODO: adjust error codes based on specific responses
            Self::FitChangeFailed(_, _) => (StatusCode::BAD_REQUEST, "FIT-000"),
            ////////////////////////////////////////////////////////////////////////////////////////
            // Item-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathItemParseFailed(_) => (StatusCode::NOT_FOUND, "ITM-001"),
            Self::PathItemNotFound(_) => (StatusCode::NOT_FOUND, "ITM-002"),
            // TODO: adjust error codes based on specific responses
            Self::ItemAddFailed(_) => (StatusCode::BAD_REQUEST, "ITM-003"),
            // TODO: adjust error codes based on specific responses
            Self::ItemChangeFailed(_) => (StatusCode::BAD_REQUEST, "ITM-004"),
            Self::ItemRemoveFailed(rs::err::RemoveItemError(rs::err::ItemRemoveItemError::ItemRemoveFailed(
                rs::err::core::RemoveItemError::UnremovableAutocharge,
            ))) => (StatusCode::FORBIDDEN, "ACH-001"),
        }
    }
    fn get_cmd_index(&self) -> Option<usize> {
        match self {
            Self::BatchParseFailed(index, _) => Some(*index),
            Self::BackrefRenderFailed(index, _) => Some(*index),
            Self::SolChangeFailed(index, _) => Some(*index),
            Self::FitChangeFailed(index, _) => Some(*index),
            _ => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<QueryRejection> for ApiError {
    fn from(query_error: QueryRejection) -> Self {
        Self::Query(query_error)
    }
}
impl From<JsonRejection> for ApiError {
    fn from(json_error: JsonRejection) -> Self {
        Self::Json(json_error)
    }
}
impl From<rs::err::ChangeSolError> for ApiError {
    fn from(rs_error: rs::err::ChangeSolError) -> Self {
        match rs_error {
            rs::err::ChangeSolError::RenderFailed(index, br_err) => Self::BackrefRenderFailed(index, br_err),
            rs::err::ChangeSolError::ExecFailed(index, exec_err) => Self::SolChangeFailed(index, exec_err),
        }
    }
}
impl From<rs::err::ChangeFitError> for ApiError {
    fn from(rs_error: rs::err::ChangeFitError) -> Self {
        match rs_error {
            rs::err::ChangeFitError::RenderFailed(index, br_err) => Self::BackrefRenderFailed(index, br_err),
            rs::err::ChangeFitError::ExecFailed(index, exec_err) => Self::FitChangeFailed(index, exec_err),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (http_code, api_code) = self.get_codes();
        let cmd_index = self.get_cmd_index();
        let payload = ApiErrorResponse {
            code: api_code,
            message: self.to_string(),
            cmd_index,
        };
        (http_code, Json(payload)).into_response()
    }
}
