use crate::phb::parsing::ReadParseFailReason;

#[derive(thiserror::Error, Debug)]
pub enum PhbFsEdhError {
    /// Handler is unable to read data.
    ///
    /// Includes suffix and error source.
    #[error("{0} reading failed")]
    Read(String, #[source] std::io::Error),
    /// Handler is unable to parse data.
    ///
    /// Includes suffix and error source.
    #[error("{0} parsing failed")]
    Parse(String, #[source] serde_json::Error),
    /// Handler is unable to find client version in metadata.
    #[error("unable to find client build field")]
    NoClientBuild,
}
impl PhbFsEdhError {
    pub(super) fn from_io(error: std::io::Error, path: String) -> Self {
        Self::Read(path, error)
    }
    pub(super) fn from_read_parse(error: ReadParseFailReason, path: String) -> Self {
        match error {
            ReadParseFailReason::Read(error) => Self::Read(path, error),
            ReadParseFailReason::Parse(error) => Self::Parse(path, error),
        }
    }
}
impl From<PhbFsEdhError> for rc::ed::err::EveDataHandlerError {
    fn from(error: PhbFsEdhError) -> Self {
        Self::new(error)
    }
}
