use serde_json;
use std::error;
use std::fmt;
use std::io;

pub(super) trait FromPathErr<T> {
    fn from_path_err<P: Into<String>>(err: T, path: P) -> Self;
}

#[derive(Debug)]
pub(super) struct Error {
    pub(super) msg: String,
}

impl Error {
    pub fn new<P: Into<String>>(msg: P) -> Error {
        Error { msg: msg.into() }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl FromPathErr<io::Error> for Error {
    fn from_path_err<P: Into<String>>(err: io::Error, path: P) -> Self {
        Error::new(format!("{} reading failed: {}", path.into(), err))
    }
}

impl FromPathErr<serde_json::Error> for Error {
    fn from_path_err<P: Into<String>>(err: serde_json::Error, path: P) -> Self {
        Error::new(format!("{} parsing failed: {}", path.into(), err))
    }
}
