use url;

use crate::util::{Error, ErrorKind};

pub(in crate::handler_http) trait FromSuffix<T> {
    fn from_suffix(err: T, suffix: &str) -> Self;
}
impl FromSuffix<url::ParseError> for Error {
    fn from_suffix(err: url::ParseError, suffix: &str) -> Self {
        Error::new(ErrorKind::HttpSuffixJoinFailed(suffix.to_string(), format!("{err}")))
    }
}
impl FromSuffix<reqwest::Error> for Error {
    fn from_suffix(err: reqwest::Error, suffix: &str) -> Self {
        if err.is_decode() {
            Error::new(ErrorKind::HttpSuffixParseFailed(suffix.to_string(), format!("{err}")))
        } else {
            Error::new(ErrorKind::HttpSuffixFetchFailed(suffix.to_string(), format!("{err}")))
        }
    }
}
impl FromSuffix<serde_json::Error> for Error {
    fn from_suffix(err: serde_json::Error, suffix: &str) -> Self {
        Error::new(ErrorKind::HttpSuffixParseFailed(suffix.to_string(), format!("{err}")))
    }
}
