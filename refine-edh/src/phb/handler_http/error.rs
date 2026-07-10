use crate::phb::parsing::ReadParseFailReason;

#[derive(thiserror::Error, Debug)]
pub enum PhbHttpEdhInitError {
    /// HTTP handler cannot use passed URL as base.
    ///
    /// Includes passed URL and text description of failure.
    #[error("invalid base URL \"{0}\": {1}")]
    PhbHttpInvalidBaseUrl(String, String),
}

#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub(super) enum PhbHttpEdhError {
    /// HTTP handler is unable to join base URL and suffix.
    ///
    /// Includes suffix and text description of failure.
    #[error("{0} is failed to be joined to base URL: {1}")]
    JoinFailed(String, String),
    /// HTTP handler is unable to fetch data.
    ///
    /// Includes suffix and text description of failure.
    #[error("{0} fetching failed: {1}")]
    FetchFailed(String, String),
    /// HTTP handler is unable to parse data.
    ///
    /// Includes suffix and text description of failure.
    #[error("{0} parsing failed: {1}")]
    ParseFailed(String, String),
}
impl PhbHttpEdhError {
    pub(super) fn from_url(error: url::ParseError, suffix: &str) -> Self {
        PhbHttpEdhError::JoinFailed(suffix.to_string(), error.to_string())
    }
    pub(super) fn from_reqwest(error: reqwest::Error, suffix: &str) -> Self {
        match error.is_decode() {
            true => PhbHttpEdhError::ParseFailed(suffix.to_string(), error.to_string()),
            false => PhbHttpEdhError::FetchFailed(suffix.to_string(), error.to_string()),
        }
    }
    pub(super) fn from_read_parse(error: ReadParseFailReason, suffix: &str) -> Self {
        match error {
            ReadParseFailReason::ReadFailed(message) => Self::FetchFailed(suffix.to_string(), message),
            ReadParseFailReason::ParseFailed(message) => Self::ParseFailed(suffix.to_string(), message),
        }
    }
}
