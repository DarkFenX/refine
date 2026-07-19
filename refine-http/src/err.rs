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
}

#[derive(Serialize)]
struct ApiErrorResponse {
    code: String,
    message: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Codes & messages
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ApiError {
    fn get_http_code(&self) -> StatusCode {
        match self {
            Self::Query(_) => StatusCode::BAD_REQUEST,
            Self::Json(_) => StatusCode::BAD_REQUEST,
            Self::BatchParseFailed(_, _) => StatusCode::BAD_REQUEST,
            // Source-related
            Self::SrcAddFailed(rs_err) => match rs_err {
                rs::src::err::AddSrcError::SrcAliasNotAvailable(_) => StatusCode::FORBIDDEN,
                rs::src::err::AddSrcError::EdhInitFailed(_) => StatusCode::BAD_REQUEST,
                rs::src::err::AddSrcError::SrcInitFailed(_) => StatusCode::UNPROCESSABLE_ENTITY,
            },
            Self::SrcRemoveFailed(rs_err) => match rs_err {
                rs::src::err::RemoveSrcError::SrcNotFound(_) => StatusCode::NOT_FOUND,
            },
            Self::PathSrcNotFound(_) => StatusCode::NOT_FOUND,
            // Solar system-related
            Self::SolAddFailed(rs_err) => match rs_err {
                rs::err::AddSolError::GetSrcFailed(_) => StatusCode::BAD_REQUEST,
            },
            Self::SolChangeFailed(rs_err) => match rs_err {
                rs::err::ChangeSolError::RenderFailed(_, _) => StatusCode::BAD_REQUEST,
                // TODO: adjust error codes based on specific responses
                rs::err::ChangeSolError::ExecFailed(_, _) => StatusCode::BAD_REQUEST,
            },
            Self::SolRemoveFailed(rs_err) => match rs_err {
                rs::err::RemoveSolError::SolNotFound(_) => StatusCode::NOT_FOUND,
            },
            Self::PathSolParseFailed(_) => StatusCode::NOT_FOUND,
            Self::PathSolNotFound(_) => StatusCode::NOT_FOUND,
            // Fit-related
            Self::FitAddFailed(rs_err) => match rs_err {
                rs::err::AddFitError::FleetSetFailed(_) => StatusCode::BAD_REQUEST,
            },
            Self::FitChangeFailed(rs_err) => match rs_err {
                rs::err::ChangeFitError::RenderFailed(_, _) => StatusCode::BAD_REQUEST,
                // TODO: adjust error codes based on specific responses
                rs::err::ChangeFitError::ExecFailed(_, _) => StatusCode::BAD_REQUEST,
            },
            Self::PathFitParseFailed(_) => StatusCode::NOT_FOUND,
            Self::PathFitNotFound(_) => StatusCode::NOT_FOUND,
            // Item-related
            // TODO: adjust error codes based on specific responses
            Self::ItemAddFailed(_) => StatusCode::BAD_REQUEST,
        }
    }
    fn get_api_code(&self) -> &str {
        match self {
            Self::Query(_) => "PRM-001",
            Self::Json(_) => "JSN-001",
            Self::BatchParseFailed(_, _) => "JSN-002",
            // Source-related
            Self::SrcAddFailed(rs_err) => match rs_err {
                rs::src::err::AddSrcError::SrcAliasNotAvailable(_) => "SRC-001",
                rs::src::err::AddSrcError::EdhInitFailed(_) => "EDH-001",
                rs::src::err::AddSrcError::SrcInitFailed(_) => "SIN-001",
            },
            Self::SrcRemoveFailed(rs_err) => match rs_err {
                rs::src::err::RemoveSrcError::SrcNotFound(_) => "SRC-004",
            },
            Self::PathSrcNotFound(rs_err) => match rs_err {
                rs::src::err::GetSrcError::SrcNotFound(_) => "SRC-002",
                rs::src::err::GetSrcError::DefaultNotDefined => "SRC-003",
            },
            // Solar system-related
            Self::SolAddFailed(rs_err) => match rs_err {
                rs::err::AddSolError::GetSrcFailed(_) => "SOL-001",
            },
            Self::SolChangeFailed(rs_err) => match rs_err {
                // TODO: adjust error codes based on specific responses
                rs::err::ChangeSolError::RenderFailed(_, _) => "SOL-004",
                rs::err::ChangeSolError::ExecFailed(_, _) => "SOL-004",
            },
            Self::SolRemoveFailed(rs_err) => match rs_err {
                rs::err::RemoveSolError::SolNotFound(_) => "SOL-005",
            },
            Self::PathSolParseFailed(_) => "SOL-002",
            Self::PathSolNotFound(rs_err) => match rs_err {
                rs::err::GetSolError::SolNotFound(_) => "SOL-003",
            },
            // Fit-related
            Self::FitAddFailed(rs_err) => match rs_err {
                rs::err::AddFitError::FleetSetFailed(_) => "FIT-001",
            },
            Self::FitChangeFailed(rs_err) => match rs_err {
                // TODO: adjust error codes based on specific responses
                rs::err::ChangeFitError::RenderFailed(_, _) => "FIT-004",
                rs::err::ChangeFitError::ExecFailed(_, _) => "FIT-004",
            },
            Self::PathFitParseFailed(_) => "FIT-002",
            Self::PathFitNotFound(_) => "FIT-003",
            // Item-related
            // TODO: adjust error codes based on specific responses
            Self::ItemAddFailed(_) => "ITM-001",
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
        let http_code = self.get_http_code();
        let api_code = self.get_api_code().to_string();
        let api_message = self.to_string();
        let payload = ApiErrorResponse {
            code: api_code,
            message: api_message,
        };
        (http_code, Json(payload)).into_response()
    }
}
