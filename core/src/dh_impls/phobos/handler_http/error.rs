use url;

use crate::util::Error;

pub(super) trait FromSuffix<T> {
    fn from_suffix<U: Into<String>>(err: T, suffix: U) -> Self;
}
impl FromSuffix<url::ParseError> for Error {
    fn from_suffix<T: Into<String>>(err: url::ParseError, suffix: T) -> Self {
        Error::new(format!("{} is failed to be parsed as URL: {}", suffix.into(), err))
    }
}
impl FromSuffix<reqwest::Error> for Error {
    fn from_suffix<T: Into<String>>(err: reqwest::Error, suffix: T) -> Self {
        if err.is_decode() {
            Error::new(format!("{} parsing failed: {}", suffix.into(), err))
        } else {
            Error::new(format!("{} fetching failed: {}", suffix.into(), err))
        }
    }
}
impl FromSuffix<serde_json::Error> for Error {
    fn from_suffix<T: Into<String>>(err: serde_json::Error, suffix: T) -> Self {
        Error::new(format!("{} parsing failed: {}", suffix.into(), err))
    }
}
