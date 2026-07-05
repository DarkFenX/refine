use axum::{Json, extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::err::HBrError;

pub(crate) enum HApiError {
    JsonFailure(JsonRejection),
    BridgeFailure(HBrError),
}

#[derive(Serialize)]
struct HApiErrorResponse {
    code: String,
    message: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HApiError {
    pub(crate) fn from_bridge(bridge_error: HBrError) -> Self {
        Self::BridgeFailure(bridge_error)
    }
}
impl From<JsonRejection> for HApiError {
    fn from(json_rejection: JsonRejection) -> Self {
        Self::JsonFailure(json_rejection)
    }
}

impl IntoResponse for HApiError {
    fn into_response(self) -> axum::response::Response {
        let (http_code, api_code, message) = match self {
            HApiError::JsonFailure(json_rejection) => (
                StatusCode::BAD_REQUEST,
                "JSN-001".to_string(),
                json_rejection.body_text(),
            ),
            HApiError::BridgeFailure(br_err) => (StatusCode::BAD_REQUEST, br_err.get_code(), br_err.to_string()),
        };
        let payload = HApiErrorResponse {
            code: api_code,
            message,
        };
        (http_code, Json(payload)).into_response()
    }
}
