use url;

use crate::util::IntError;

pub(in crate::edh_impls::phobos::handler_http) trait FromSuffix<T> {
    fn from_suffix(err: T, suffix: &str) -> Self;
}
impl FromSuffix<url::ParseError> for IntError {
    fn from_suffix(err: url::ParseError, suffix: &str) -> Self {
        IntError::new(format!("{} is failed to be parsed as URL: {}", suffix, err))
    }
}
impl FromSuffix<reqwest::Error> for IntError {
    fn from_suffix(err: reqwest::Error, suffix: &str) -> Self {
        if err.is_decode() {
            IntError::new(format!("{} parsing failed: {}", suffix, err))
        } else {
            IntError::new(format!("{} fetching failed: {}", suffix, err))
        }
    }
}
impl FromSuffix<serde_json::Error> for IntError {
    fn from_suffix(err: serde_json::Error, suffix: &str) -> Self {
        IntError::new(format!("{} parsing failed: {}", suffix, err))
    }
}
