use serde::Serialize;

use crate::err::HBrError;

#[derive(Serialize)]
pub(in crate::handlers) struct HSingleErr {
    code: String,
    message: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSingleErr {
    pub(in crate::handlers) fn from_bridge(bridge_error: HBrError) -> Self {
        Self {
            code: bridge_error.get_api_code(),
            message: bridge_error.to_string(),
        }
    }
}
