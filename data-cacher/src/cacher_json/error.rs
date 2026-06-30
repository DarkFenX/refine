#[derive(thiserror::Error, Debug)]
pub(super) enum JsonZfileAdcError {
    #[error("unable to read cache: {0}")]
    ReadFailed(String),
    #[error("unable to parse cache data: {0}")]
    ParseFailed(String),
}
impl From<struson::reader::ReaderError> for JsonZfileAdcError {
    fn from(error: struson::reader::ReaderError) -> Self {
        match error {
            struson::reader::ReaderError::IoError { .. } => Self::ReadFailed(error.to_string()),
            _ => Self::ParseFailed(error.to_string()),
        }
    }
}
impl From<struson::serde::DeserializerError> for JsonZfileAdcError {
    fn from(error: struson::serde::DeserializerError) -> Self {
        match error {
            struson::serde::DeserializerError::ReaderError(reader_error) => reader_error.into(),
            _ => Self::ParseFailed(error.to_string()),
        }
    }
}
