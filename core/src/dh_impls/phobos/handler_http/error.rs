use url;

use crate::IntError;

pub(super) trait FromSuffix<T> {
    fn from_suffix<U: Into<String>>(err: T, suffix: U) -> Self;
}
impl FromSuffix<url::ParseError> for IntError {
    fn from_suffix<T: Into<String>>(err: url::ParseError, suffix: T) -> Self {
        IntError::new(format!("{} is failed to be parsed as URL: {}", suffix.into(), err))
    }
}
impl FromSuffix<reqwest::Error> for IntError {
    fn from_suffix<T: Into<String>>(err: reqwest::Error, suffix: T) -> Self {
        if err.is_decode() {
            IntError::new(format!("{} parsing failed: {}", suffix.into(), err))
        } else {
            IntError::new(format!("{} fetching failed: {}", suffix.into(), err))
        }
    }
}
impl FromSuffix<serde_json::Error> for IntError {
    fn from_suffix<T: Into<String>>(err: serde_json::Error, suffix: T) -> Self {
        IntError::new(format!("{} parsing failed: {}", suffix.into(), err))
    }
}
