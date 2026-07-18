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
            // Source-related
            Self::SrcAddFailed(rs_err) => match rs_err {
                rs::src::err::AddSrcError::SrcAliasNotAvailable(_) => StatusCode::FORBIDDEN,
                rs::src::err::AddSrcError::EdhInitFailed(_) => StatusCode::BAD_REQUEST,
                rs::src::err::AddSrcError::SrcInitFailed(_) => StatusCode::UNPROCESSABLE_ENTITY,
            },
            // The only way for remove to fail is when source gets removed mid-handling
            // (after get() but before remove()), so treat them equally
            Self::SrcRemoveFailed(_) => StatusCode::NOT_FOUND,
            Self::PathSrcNotFound(_) => StatusCode::NOT_FOUND,
            // Solar system-related
            Self::SolAddFailed(_) => StatusCode::BAD_REQUEST,
        }
    }
    fn get_api_code(&self) -> &str {
        match self {
            Self::Query(_) => "PRM-001",
            Self::Json(_) => "JSN-001",
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
