#[derive(thiserror::Error, Debug)]
pub(super) enum JsonZfileAdcDataReadError {
    #[error("reading failed: {0}")]
    ReadFailed(String),
    #[error("parsing failed: {0}")]
    ParseFailed(String),
}
impl From<struson::reader::ReaderError> for JsonZfileAdcDataReadError {
    fn from(error: struson::reader::ReaderError) -> Self {
        match error {
            struson::reader::ReaderError::IoError { .. } => Self::ReadFailed(error.to_string()),
            _ => Self::ParseFailed(error.to_string()),
        }
    }
}
impl From<struson::serde::DeserializerError> for JsonZfileAdcDataReadError {
    fn from(error: struson::serde::DeserializerError) -> Self {
        match error {
            struson::serde::DeserializerError::ReaderError(reader_error) => reader_error.into(),
            _ => Self::ParseFailed(error.to_string()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub(super) enum JsonZfileAdcFpReadError {
    #[error("reading failed: {0}")]
    ReadFailed(String),
}

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub(super) enum JsonZfileAdcWriteError {
    #[error("unable to create directory: {0}")]
    CreateDirFailed(String),
    #[error("unable to write data: {0}")]
    DataWriteFailed(String),
    #[error("unable to serialize data: {0}")]
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
