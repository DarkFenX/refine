#[derive(thiserror::Error, Debug)]
pub(super) enum JsonZfileAdcDataReadError {
    #[error("reading failed: {0}")]
    ReadFailed(String),
    #[error("parsing failed: {0}")]
    ParseFailed(String),
}
impl From<serde_json::Error> for JsonZfileAdcDataReadError {
    fn from(error: serde_json::Error) -> Self {
        match error.is_io() {
            true => Self::ReadFailed(error.to_string()),
            false => Self::ParseFailed(error.to_string()),
        }
    }
}
impl From<JsonZfileAdcDataReadError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: JsonZfileAdcDataReadError) -> Self {
        Self::new(error)
    }
}

#[derive(thiserror::Error, Debug)]
pub(super) enum JsonZfileAdcFpReadError {
    #[error("reading failed: {0}")]
    ReadFailed(String),
}
impl From<JsonZfileAdcFpReadError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: JsonZfileAdcFpReadError) -> Self {
        Self::new(error)
    }
}

#[expect(clippy::enum_variant_names)]
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
impl From<serde_json::Error> for JsonZfileAdcWriteError {
    fn from(error: serde_json::Error) -> Self {
        match error.is_io() {
            true => Self::DataWriteFailed(error.to_string()),
            false => Self::DataSerializeFailed(error.to_string()),
        }
    }
}
impl From<JsonZfileAdcWriteError> for rc::ad::err::AdaptedDataCacherError {
    fn from(error: JsonZfileAdcWriteError) -> Self {
        Self::new(error)
    }
}
