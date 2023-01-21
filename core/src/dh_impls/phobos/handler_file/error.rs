use std::io;

use crate::util::Error;

pub(super) trait FromPath<T> {
    fn from_path<U: Into<String>>(err: T, path: U) -> Self;
}
impl FromPath<io::Error> for Error {
    fn from_path<T: Into<String>>(err: io::Error, path: T) -> Self {
        Error::new(format!("{} reading failed: {}", path.into(), err))
    }
}
impl FromPath<serde_json::Error> for Error {
    fn from_path<T: Into<String>>(err: serde_json::Error, path: T) -> Self {
        if err.is_io() {
            Error::new(format!("{} reading failed: {}", path.into(), err))
        } else {
            Error::new(format!("{} parsing failed: {}", path.into(), err))
        }
    }
}
