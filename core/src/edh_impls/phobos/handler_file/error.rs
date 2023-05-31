use std::io;

use crate::util::IntError;

pub(in crate::edh_impls::phobos::handler_file) trait FromPath<T> {
    fn from_path(err: T, path: &str) -> Self;
}
impl FromPath<io::Error> for IntError {
    fn from_path(err: io::Error, path: &str) -> Self {
        IntError::new(format!("{} reading failed: {}", path, err))
    }
}
impl FromPath<serde_json::Error> for IntError {
    fn from_path(err: serde_json::Error, path: &str) -> Self {
        if err.is_io() {
            IntError::new(format!("{} reading failed: {}", path, err))
        } else {
            IntError::new(format!("{} parsing failed: {}", path, err))
        }
    }
}
