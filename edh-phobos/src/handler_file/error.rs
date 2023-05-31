use std::io;

use crate::util::{Error, ErrorKind};

pub(in crate::handler_file) trait FromPath<T> {
    fn from_path(err: T, path: &str) -> Self;
}
impl FromPath<io::Error> for Error {
    fn from_path(err: io::Error, path: &str) -> Self {
        Error::new(ErrorKind::FileSuffixReadFailed(path.to_string(), format!("{err}")))
    }
}
impl FromPath<serde_json::Error> for Error {
    fn from_path(err: serde_json::Error, path: &str) -> Self {
        if err.is_io() {
            Error::new(ErrorKind::FileSuffixReadFailed(path.to_string(), format!("{err}")))
        } else {
            Error::new(ErrorKind::FileSuffixParseFailed(path.to_string(), format!("{err}")))
        }
    }
}
