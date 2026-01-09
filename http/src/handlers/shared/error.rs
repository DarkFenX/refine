use serde::Serialize;

use crate::bridge::HBrError;

#[derive(Serialize)]
pub(in crate::handlers) struct HSingleErr {
    code: String,
    message: String,
}
impl From<HBrError> for HSingleErr {
    fn from(sol_error: HBrError) -> Self {
        Self {
            code: sol_error.get_code(),
            message: sol_error.to_string(),
        }
    }
}
