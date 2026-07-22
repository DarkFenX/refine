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
    #[error("command #{0} failed: {1}")]
    BatchParseFailed(usize, String),
    // Source-related
    #[error("failed to add source: {0}")]
    SrcAddFailed(#[from] rs::src::err::AddSrcError),
    #[error("failed to remove source: {0}")]
    SrcRemoveFailed(#[from] rs::src::err::RemoveSrcError),
    #[error("failed to get source: {0}")]
    PathSrcNotFound(#[from] rs::src::err::GetSrcError),
    // Solar system-related
    #[error("failed to add solar system: {0}")]
    SolAddFailed(#[from] rs::err::AddSolError),
    #[error("failed to change solar system: {0}")]
    SolChangeFailed(#[from] rs::err::ChangeSolError),
    #[error("failed to remove solar system: {0}")]
    SolRemoveFailed(#[from] rs::err::RemoveSolError),
    #[error("failed to get solar system: {0}")]
    PathSolParseFailed(#[from] rs::err::ParseSolarSystemIdError),
    #[error("failed to get solar system: {0}")]
    PathSolNotFound(#[from] rs::err::GetSolError),
    #[error("failed to switch solar system source: {0}")]
    SolSrcSwitch(#[from] rs::err::SolSwitchSrcError),
    // Fleet-related
    #[error("failed to add fleet: {0}")]
    FleetAddFailed(#[from] rs::err::AddFleetError),
    #[error("failed to change fleet: {0}")]
    FleetChangeFailed(#[from] rs::err::ChangeFleetError),
    #[error("failed to get fleet: {0}")]
    PathFleetParseFailed(#[from] rs::err::ParseFleetIdError),
    #[error("failed to get fleet: {0}")]
    PathFleetNotFound(#[from] rs::err::GetFleetError),
    // Fit-related
    #[error("failed to add fit: {0}")]
    FitAddFailed(#[from] rs::err::AddFitError),
    #[error("failed to change fit: {0}")]
    FitChangeFailed(#[from] rs::err::ChangeFitError),
    #[error("failed to get fit: {0}")]
    PathFitParseFailed(#[from] rs::err::ParseFitIdError),
    #[error("failed to get fit: {0}")]
    PathFitNotFound(#[from] rs::err::GetFitError),
    // Item-related
    #[error("failed to add item: {0}")]
    ItemAddFailed(#[from] rs::err::AddItemEnumError),
    #[error("failed to change item: {0}")]
    ItemChangeFailed(#[from] rs::err::ChangeItemEnumError),
    #[error("failed to remove item: {0}")]
    ItemRemoveFailed(#[from] rs::err::RemoveItemError),
    #[error("failed to get item: {0}")]
    PathItemParseFailed(#[from] rs::err::ParseItemIdError),
    #[error("failed to get item: {0}")]
    PathItemNotFound(#[from] rs::err::GetItemError),
}

#[derive(Serialize)]
struct ApiErrorResponse {
    code: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmd_idx: Option<usize>,
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
            ////////////////////////////////////////////////////////////////////////////////////////
            // Source-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::SrcAddFailed(rs_err) => match rs_err {
                rs::src::err::AddSrcError::SrcAliasNotAvailable(_) => (StatusCode::FORBIDDEN, "SRC-001"),
                rs::src::err::AddSrcError::EdhInitFailed(_) => (StatusCode::BAD_REQUEST, "EDH-001"),
                rs::src::err::AddSrcError::SrcInitFailed(_) => (StatusCode::UNPROCESSABLE_ENTITY, "SNT-001"),
            },
            Self::PathSrcNotFound(rs_err) => match rs_err {
                rs::src::err::GetSrcError::SrcNotFound(_) => (StatusCode::NOT_FOUND, "SRC-002"),
                rs::src::err::GetSrcError::DefaultNotDefined => (StatusCode::NOT_FOUND, "SRC-003"),
            },
            Self::SrcRemoveFailed(rs_err) => match rs_err {
                rs::src::err::RemoveSrcError::SrcNotFound(_) => (StatusCode::NOT_FOUND, "SRC-004"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Solar system-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::SolAddFailed(rs_err) => match rs_err {
                rs::err::AddSolError::GetSrcFailed(_) => (StatusCode::BAD_REQUEST, "SOL-001"),
            },
            Self::PathSolParseFailed(_) => (StatusCode::NOT_FOUND, "SOL-002"),
            Self::PathSolNotFound(rs_err) => match rs_err {
                rs::err::GetSolError::SolNotFound(_) => (StatusCode::NOT_FOUND, "SOL-003"),
            },
            Self::SolChangeFailed(rs_err) => match rs_err {
                rs::err::ChangeSolError::RenderFailed(_, br_err) => match br_err {
                    rs::err::BackrefRenderError::NotFound(_) => (StatusCode::BAD_REQUEST, "SOL-004"),
                    rs::err::BackrefRenderError::NoFitId(_) => (StatusCode::BAD_REQUEST, "SOL-005"),
                    rs::err::BackrefRenderError::NoFleetId(_) => (StatusCode::BAD_REQUEST, "SOL-006"),
                    rs::err::BackrefRenderError::NoItemId(_) => (StatusCode::BAD_REQUEST, "SOL-007"),
                    rs::err::BackrefRenderError::NoChargeItemId(_) => (StatusCode::BAD_REQUEST, "SOL-008"),
                },
                // TODO: adjust error codes based on specific responses
                rs::err::ChangeSolError::ExecFailed(_, _) => (StatusCode::BAD_REQUEST, "SOL-000"),
            },
            Self::SolRemoveFailed(rs_err) => match rs_err {
                rs::err::RemoveSolError::SolNotFound(_) => (StatusCode::NOT_FOUND, "SOL-009"),
            },
            Self::SolSrcSwitch(rs_err) => match rs_err {
                rs::err::SolSwitchSrcError::SrcGetFailed(_) => (StatusCode::BAD_REQUEST, "SOL-010"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Fleet-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::FleetAddFailed(rs_err) => match rs_err {
                rs::err::AddFleetError::FitAddFailed(_) => (StatusCode::BAD_REQUEST, "FLT-001"),
            },
            Self::PathFleetParseFailed(_) => (StatusCode::NOT_FOUND, "FLT-002"),
            Self::PathFleetNotFound(_) => (StatusCode::NOT_FOUND, "FLT-003"),
            Self::FleetChangeFailed(_) => (StatusCode::BAD_REQUEST, "FLT-004"),
            ////////////////////////////////////////////////////////////////////////////////////////
            // Fit-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::FitAddFailed(rs_err) => match rs_err {
                rs::err::AddFitError::FleetSetFailed(_) => (StatusCode::BAD_REQUEST, "FIT-001"),
            },
            Self::PathFitParseFailed(_) => (StatusCode::NOT_FOUND, "FIT-002"),
            Self::PathFitNotFound(_) => (StatusCode::NOT_FOUND, "FIT-003"),
            Self::FitChangeFailed(rs_err) => match rs_err {
                rs::err::ChangeFitError::RenderFailed(_, br_err) => match br_err {
                    rs::err::BackrefRenderError::NotFound(_) => (StatusCode::BAD_REQUEST, "FIT-004"),
                    rs::err::BackrefRenderError::NoFitId(_) => (StatusCode::BAD_REQUEST, "FIT-005"),
                    rs::err::BackrefRenderError::NoFleetId(_) => (StatusCode::BAD_REQUEST, "FIT-006"),
                    rs::err::BackrefRenderError::NoItemId(_) => (StatusCode::BAD_REQUEST, "FIT-007"),
                    rs::err::BackrefRenderError::NoChargeItemId(_) => (StatusCode::BAD_REQUEST, "FIT-008"),
                },
                // TODO: adjust error codes based on specific responses
                rs::err::ChangeFitError::ExecFailed(_, _) => (StatusCode::BAD_REQUEST, "FIT-000"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Item-related
            ////////////////////////////////////////////////////////////////////////////////////////
            // TODO: adjust error codes based on specific responses
            Self::ItemAddFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
            Self::PathItemParseFailed(_) => (StatusCode::NOT_FOUND, "ITM-002"),
            Self::PathItemNotFound(_) => (StatusCode::NOT_FOUND, "ITM-003"),
            // TODO: adjust error codes based on specific responses
            Self::ItemChangeFailed(_) => (StatusCode::BAD_REQUEST, "ITM-004"),
            Self::ItemRemoveFailed(rs::err::RemoveItemError(rs::err::ItemRemoveItemError::ItemRemoveFailed(
                rs::err::core::RemoveItemError::UnremovableAutocharge,
            ))) => (StatusCode::FORBIDDEN, "ACH-001"),
        }
    }
    fn get_cmd_idx(&self) -> Option<usize> {
        match self {
            Self::BatchParseFailed(index, _) => Some(*index),
            Self::SolChangeFailed(rs::err::ChangeSolError::RenderFailed(index, _)) => Some(*index),
            Self::SolChangeFailed(rs::err::ChangeSolError::ExecFailed(index, _)) => Some(*index),
            Self::FitChangeFailed(rs::err::ChangeFitError::RenderFailed(index, _)) => Some(*index),
            Self::FitChangeFailed(rs::err::ChangeFitError::ExecFailed(index, _)) => Some(*index),
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

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (http_code, api_code) = self.get_codes();
        let cmd_idx = self.get_cmd_idx();
        let payload = ApiErrorResponse {
            code: api_code,
            message: self.to_string(),
            cmd_idx,
        };
        (http_code, Json(payload)).into_response()
    }
}
