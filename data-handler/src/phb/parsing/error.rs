#[derive(thiserror::Error, Debug)]
pub(in crate::phb) enum ReadParseError {
    #[error("reading failed: {0}")]
    ReadFailed(String),
    #[error("parsing failed: {0}")]
    ParseFailed(String),
}
impl From<struson::reader::ReaderError> for ReadParseError {
    fn from(error: struson::reader::ReaderError) -> Self {
        match error {
            struson::reader::ReaderError::IoError { .. } => Self::ReadFailed(error.to_string()),
            _ => Self::ParseFailed(error.to_string()),
        }
    }
}
impl From<struson::serde::DeserializerError> for ReadParseError {
    fn from(error: struson::serde::DeserializerError) -> Self {
        match error {
            struson::serde::DeserializerError::ReaderError(reader_error) => reader_error.into(),
            _ => Self::ParseFailed(error.to_string()),
        }
    }
}
impl From<Box<dyn std::error::Error>> for ReadParseError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        let error = match error.downcast::<struson::reader::ReaderError>() {
            Ok(reader_error) => return (*reader_error).into(),
            Err(error) => error,
        };
        match error.downcast::<struson::serde::DeserializerError>() {
            Ok(de_error) => (*de_error).into(),
            Err(error) => Self::ParseFailed(error.to_string()),
        }
    }
}
