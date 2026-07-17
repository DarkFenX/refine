use crate::phb::parsing::ReadParseFailReason;

#[derive(thiserror::Error, Debug)]
pub(super) enum PhbFsEdhError {
    /// Filesystem handler is unable to read data.
    ///
    /// Includes suffix and text description of failure.
    #[error("{0} reading failed: {1}")]
    ReadFailed(String, String),
    /// Filesystem handler is unable to parse data.
    ///
    /// Includes suffix and text description of failure.
    #[error("{0} parsing failed: {1}")]
    ParseFailed(String, String),
    /// Filesystem handler is unable to find client version in metadata.
    #[error("unable to find client build field")]
    NoClientBuild,
}
impl PhbFsEdhError {
    pub(super) fn from_io(error: std::io::Error, path: String) -> Self {
        Self::ReadFailed(path, error.to_string())
    }
    pub(super) fn from_read_parse(error: ReadParseFailReason, path: String) -> Self {
        match error {
            ReadParseFailReason::ReadFailed(message) => Self::ReadFailed(path, message),
            ReadParseFailReason::ParseFailed(message) => Self::ParseFailed(path, message),
        }
    }
}
