#[derive(thiserror::Error, Debug)]
pub(super) enum JsonZfileAdcReadError {
    #[error("unable to read cache: {0}")]
    ReadFailed(String),
    #[error("unable to parse cache data: {0}")]
    ParseFailed(String),
}
impl From<struson::reader::ReaderError> for JsonZfileAdcReadError {
    fn from(error: struson::reader::ReaderError) -> Self {
        match error {
            struson::reader::ReaderError::IoError { .. } => Self::ReadFailed(error.to_string()),
            _ => Self::ParseFailed(error.to_string()),
        }
    }
}
impl From<struson::serde::DeserializerError> for JsonZfileAdcReadError {
    fn from(error: struson::serde::DeserializerError) -> Self {
        match error {
            struson::serde::DeserializerError::ReaderError(reader_error) => reader_error.into(),
            _ => Self::ParseFailed(error.to_string()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub(super) enum JsonZfileAdcWriteError {
    #[error("unable to create cache folder: {0}")]
    CreateFolderFailed(String),
    #[error("unable to write cache data: {0}")]
    DataWriteFailed(String),
    #[error("unable to serialize cache data: {0}")]
    DataSerializeFailed(String),
    #[error("unable to write fingerprint: {0}")]
    FpWriteFailed(String),
}
impl From<std::io::Error> for JsonZfileAdcWriteError {
    fn from(error: std::io::Error) -> Self {
        Self::DataWriteFailed(error.to_string())
    }
}
impl From<struson::serde::SerializerError> for JsonZfileAdcWriteError {
    fn from(error: struson::serde::SerializerError) -> Self {
        Self::DataSerializeFailed(error.to_string())
    }
}
