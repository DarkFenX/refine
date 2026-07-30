#[derive(thiserror::Error, Debug)]
pub(super) enum PostcardZfileAdcDataReadError {
    #[error("reading failed: {0}")]
    ReadFailed(String),
    #[error("parsing failed: {0}")]
    ParseFailed(String),
}
impl From<postcard::Error> for PostcardZfileAdcDataReadError {
    fn from(error: postcard::Error) -> Self {
        Self::ParseFailed(error.to_string())
    }
}

#[derive(thiserror::Error, Debug)]
pub(super) enum PostcardZfileAdcFpReadError {
    #[error("reading failed: {0}")]
    ReadFailed(String),
}

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub(super) enum PostcardZfileAdcWriteError {
    #[error("unable to create directory: {0}")]
    CreateDirFailed(String),
    #[error("unable to write data: {0}")]
    DataWriteFailed(String),
    #[error("unable to serialize data: {0}")]
    DataSerializeFailed(String),
    #[error("unable to write fingerprint: {0}")]
    FpWriteFailed(String),
}
impl From<std::io::Error> for PostcardZfileAdcWriteError {
    fn from(error: std::io::Error) -> Self {
        Self::DataWriteFailed(error.to_string())
    }
}
impl From<postcard::Error> for PostcardZfileAdcWriteError {
    fn from(error: postcard::Error) -> Self {
        Self::DataSerializeFailed(error.to_string())
    }
}
