use crate::sde::parsing::ReadParseFailReason;

#[derive(thiserror::Error, Debug)]
pub enum SdeHttpEdhInitError {
    /// Passed base URL cannot be interpreted as an URL.
    ///
    /// Includes passed URL.
    #[error("invalid base URL \"{0}\"")]
    BaseUrlParse(String, #[source] url::ParseError),
    /// Passed URL is a valid URL, but cannot be used as a base for other URLs.
    ///
    /// Includes passed URL.
    #[error("URL \"{0}\" cannot be used as base URL")]
    BaseUrlNotABase(String),
}

#[derive(thiserror::Error, Debug)]
pub enum SdeHttpEdhError {
    /// Handler is unable to join base URL and suffix.
    ///
    /// Includes suffix and error source.
    #[error("{0} is failed to be joined to base URL")]
    Join(String, #[source] url::ParseError),
    /// Handler is unable to fetch data.
    ///
    /// Includes suffix and error source.
    #[error("{0} fetching failed")]
    Fetch(String, #[source] Box<dyn std::error::Error + Send + Sync>),
    /// Handler is unable to parse data.
    ///
    /// Includes suffix and error source.
    #[error("{0} parsing failed")]
    Parse(String, #[source] Box<dyn std::error::Error + Send + Sync>),
}
impl SdeHttpEdhError {
    pub(super) fn from_url(error: url::ParseError, suffix: &str) -> Self {
        Self::Join(suffix.to_string(), error)
    }
    pub(super) fn from_reqwest(error: reqwest::Error, suffix: &str) -> Self {
        match error.is_decode() {
            true => Self::Parse(suffix.to_string(), Box::new(error)),
            false => Self::Fetch(suffix.to_string(), Box::new(error)),
        }
    }
    pub(super) fn from_read_parse(error: ReadParseFailReason, suffix: &str) -> Self {
        match error {
            ReadParseFailReason::Read(error) => Self::Fetch(suffix.to_string(), Box::new(error)),
        }
    }
}
impl From<SdeHttpEdhError> for rc::ed::err::EveDataHandlerError {
    fn from(error: SdeHttpEdhError) -> Self {
        Self::new(error)
    }
}
