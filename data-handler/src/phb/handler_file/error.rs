use crate::phb::parsing::ReadParseError;

#[derive(thiserror::Error, Debug)]
pub(super) enum PhbFileEdhError {
    /// File handler is unable to read data.
    ///
    /// Includes suffix and text description of failure.
    #[error("{0} reading failed: {1}")]
    ReadFailed(String, String),
    /// File handler is unable to parse data.
    ///
    /// Includes suffix and text description of failure.
    #[error("{0} parsing failed: {1}")]
    ParseFailed(String, String),
    /// File handler is unable to find client version in metadata.
    #[error("unable to find client build field")]
    NoClientBuild,
}
impl PhbFileEdhError {
    pub(super) fn from_io(error: std::io::Error, path: String) -> Self {
        Self::ReadFailed(path, error.to_string())
    }
    pub(super) fn from_read_parse(error: ReadParseError, path: String) -> Self {
        match error {
            ReadParseError::ReadFailed(message) => Self::ReadFailed(path, message),
            ReadParseError::ParseFailed(message) => Self::ParseFailed(path, message),
        }
    }
}
