pub(in crate::phb) enum ReadParseFailReason {
    ReadFailed(String),
    ParseFailed(String),
}
impl From<struson::reader::ReaderError> for ReadParseFailReason {
    fn from(error: struson::reader::ReaderError) -> Self {
        match error {
            struson::reader::ReaderError::IoError { .. } => Self::ReadFailed(error.to_string()),
            _ => Self::ParseFailed(error.to_string()),
        }
    }
}
impl From<struson::serde::DeserializerError> for ReadParseFailReason {
    fn from(error: struson::serde::DeserializerError) -> Self {
        match error {
            struson::serde::DeserializerError::ReaderError(reader_error) => reader_error.into(),
            _ => Self::ParseFailed(error.to_string()),
        }
    }
}
