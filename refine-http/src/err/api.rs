use axum::{
    Json,
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;

use crate::err::{HBrError, HExecError};

pub(crate) enum HApiError {
    Query(QueryRejection),
    Json(JsonRejection),
    Bridge(HBrErrorPathAware),
}

pub(crate) struct HBrErrorPathAware {
    err: HBrError,
    src_in_path: bool,
    fleet_in_path: bool,
    fit_in_path: bool,
    item_in_path: bool,
}

#[derive(Serialize)]
struct HApiErrorResponse {
    code: String,
    message: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Codes & messages
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HApiError {
    fn get_http_code(&self) -> StatusCode {
        match self {
            HApiError::Query(_) => StatusCode::BAD_REQUEST,
            HApiError::Json(_) => StatusCode::BAD_REQUEST,
            HApiError::Bridge(br_err) => match &br_err.err {
                // Related to source initialization
                HBrError::EdhInitFailed(_) => StatusCode::BAD_REQUEST,
                HBrError::SrcInitFailed(_) => StatusCode::UNPROCESSABLE_ENTITY,
                HBrError::SolNotFound(_) => StatusCode::NOT_FOUND,
                HBrError::NoCoreSol => StatusCode::INTERNAL_SERVER_ERROR,
                // Source-related issues
                HBrError::SrcAliasNotAvailable(_) => StatusCode::FORBIDDEN,
                HBrError::SrcNotFound(_) if br_err.src_in_path => StatusCode::NOT_FOUND,
                // Casts happen only when those IDs are in HTTP paths; if they fail, can safely
                // assume that it's 404
                HBrError::FleetIdCastFailed(_) | HBrError::FitIdCastFailed(_) | HBrError::ItemIdCastFailed(_) => {
                    StatusCode::NOT_FOUND
                }
                HBrError::ExecFailed(exec_err) | HBrError::BatchExecFailed(_, exec_err) => match exec_err {
                    // Return 404 for fleet/fit/item not found errors only if they were part of path
                    HExecError::FleetNotFoundPrimary(_) if br_err.fleet_in_path => StatusCode::NOT_FOUND,
                    HExecError::FitNotFoundPrimary(_) if br_err.fit_in_path => StatusCode::NOT_FOUND,
                    HExecError::ItemNotFoundPrimary(_) if br_err.item_in_path => StatusCode::NOT_FOUND,
                    // Attempt to remove unremovable item is 403
                    HExecError::UnremovableAutocharge => StatusCode::FORBIDDEN,
                    // Attempt to add skill which is already on fit is 409
                    HExecError::SkillIdCollision(_) => StatusCode::CONFLICT,
                    _ => StatusCode::BAD_REQUEST,
                },
                _ => StatusCode::BAD_REQUEST,
            },
        }
    }
    fn get_api_code(&self) -> String {
        match self {
            Self::Query(_) => "PRM-001".to_string(),
            Self::Json(_) => "JSN-001".to_string(),
            Self::Bridge(br_err) => br_err.err.get_api_code(),
        }
    }
    fn get_message(&self) -> String {
        match self {
            Self::Query(query_err) => query_err.body_text(),
            Self::Json(json_err) => json_err.body_text(),
            Self::Bridge(br_err) => br_err.err.to_string(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HApiError {
    pub(crate) fn from_br_path_empty(bridge_error: HBrError) -> Self {
        Self::Bridge(HBrErrorPathAware {
            err: bridge_error,
            src_in_path: false,
            fleet_in_path: false,
            fit_in_path: false,
            item_in_path: false,
        })
    }
    pub(crate) fn from_br_path_src(bridge_error: HBrError) -> Self {
        Self::Bridge(HBrErrorPathAware {
            err: bridge_error,
            src_in_path: true,
            fleet_in_path: false,
            fit_in_path: false,
            item_in_path: false,
        })
    }
    pub(crate) fn from_br_path_sol(bridge_error: HBrError) -> Self {
        Self::Bridge(HBrErrorPathAware {
            err: bridge_error,
            src_in_path: false,
            fleet_in_path: false,
            fit_in_path: false,
            item_in_path: false,
        })
    }
    pub(crate) fn from_br_path_sol_fleet(bridge_error: HBrError) -> Self {
        Self::Bridge(HBrErrorPathAware {
            err: bridge_error,
            src_in_path: false,
            fleet_in_path: true,
            fit_in_path: false,
            item_in_path: false,
        })
    }
    pub(crate) fn from_br_path_sol_fit(bridge_error: HBrError) -> Self {
        Self::Bridge(HBrErrorPathAware {
            err: bridge_error,
            src_in_path: false,
            fleet_in_path: false,
            fit_in_path: true,
            item_in_path: false,
        })
    }
    pub(crate) fn from_br_path_sol_item(bridge_error: HBrError) -> Self {
        Self::Bridge(HBrErrorPathAware {
            err: bridge_error,
            src_in_path: false,
            fleet_in_path: false,
            fit_in_path: false,
            item_in_path: true,
        })
    }
}
impl From<QueryRejection> for HApiError {
    fn from(query_error: QueryRejection) -> Self {
        Self::Query(query_error)
    }
}
impl From<JsonRejection> for HApiError {
    fn from(json_error: JsonRejection) -> Self {
        Self::Json(json_error)
    }
}

impl IntoResponse for HApiError {
    fn into_response(self) -> axum::response::Response {
        let payload = HApiErrorResponse {
            code: self.get_api_code(),
            message: self.get_message(),
        };
        (self.get_http_code(), Json(payload)).into_response()
    }
}
