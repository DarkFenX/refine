use std::io;

use crate::IntError;

pub(super) trait FromPath<T> {
    fn from_path<U: Into<String>>(err: T, path: U) -> Self;
}
impl FromPath<io::Error> for IntError {
    fn from_path<T: Into<String>>(err: io::Error, path: T) -> Self {
        IntError::new(format!("{} reading failed: {}", path.into(), err))
    }
}
impl FromPath<serde_json::Error> for IntError {
    fn from_path<T: Into<String>>(err: serde_json::Error, path: T) -> Self {
        if err.is_io() {
            IntError::new(format!("{} reading failed: {}", path.into(), err))
        } else {
            IntError::new(format!("{} parsing failed: {}", path.into(), err))
        }
    }
}
