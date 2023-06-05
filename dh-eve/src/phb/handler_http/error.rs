use url;

use crate::util::{Error, ErrorKind};

pub(in crate::phb::handler_http) trait FromSuffix<T> {
    fn from_suffix(err: T, suffix: &str) -> Self;
}
impl FromSuffix<url::ParseError> for Error {
    fn from_suffix(err: url::ParseError, suffix: &str) -> Self {
        Error::new(ErrorKind::PhbHttpSuffixJoinFailed(suffix.to_string(), err.to_string()))
    }
}
impl FromSuffix<reqwest::Error> for Error {
    fn from_suffix(err: reqwest::Error, suffix: &str) -> Self {
        if err.is_decode() {
            Error::new(ErrorKind::PhbHttpSuffixParseFailed(suffix.to_string(), err.to_string()))
        } else {
            Error::new(ErrorKind::PhbHttpSuffixFetchFailed(suffix.to_string(), err.to_string()))
        }
    }
}
impl FromSuffix<serde_json::Error> for Error {
    fn from_suffix(err: serde_json::Error, suffix: &str) -> Self {
        Error::new(ErrorKind::PhbHttpSuffixParseFailed(suffix.to_string(), err.to_string()))
    }
}
