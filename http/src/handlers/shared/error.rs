use crate::util::HError;

#[derive(serde::Serialize)]
pub(in crate::handlers) struct HSingleErr {
    code: String,
    message: String,
}
impl From<HError> for HSingleErr {
    fn from(err: HError) -> Self {
        Self {
            code: err.get_code(),
            message: err.to_string(),
        }
    }
}
