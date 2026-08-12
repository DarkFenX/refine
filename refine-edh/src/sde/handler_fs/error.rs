use crate::sde::parsing::ReadParseFailReason;

#[derive(thiserror::Error, Debug)]
pub enum SdeFsEdhError {
    /// Handler is unable to read data.
    ///
    /// Includes file name and error source.
    #[error("{0} reading failed")]
    Read(String, #[source] std::io::Error),
    /// Handler is unable to parse data.
    ///
    /// Includes file name and error source.
    #[error("{0} parsing failed")]
    Parse(String, #[source] serde_json::Error),
    /// Handler is unable to find data build number in metadata.
    #[error("unable to find build number")]
    NoBuildNumber,
}
impl SdeFsEdhError {
    pub(super) fn from_io(error: std::io::Error, file: &str) -> Self {
        Self::Read(file.to_string(), error)
    }
    pub(super) fn from_read_parse(error: ReadParseFailReason, file: &str) -> Self {
        match error {
            ReadParseFailReason::Read(error) => Self::Read(file.to_string(), error),
            ReadParseFailReason::Parse(error) => Self::Parse(file.to_string(), error),
        }
    }
}
impl From<SdeFsEdhError> for rc::ed::err::EveDataHandlerError {
    fn from(error: SdeFsEdhError) -> Self {
        Self::new(error)
    }
}
