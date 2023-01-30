use crate::util::err_res::internal::IntResult;
use std::{error, fmt, result};

use super::internal::IntError;

/// Defines error types which are returned by the library.
#[derive(Debug)]
pub enum ErrorKind {
    DhHttpInvalidBaseUrl,
    SrcAlreadyExists,
    SrcCacheGenFailed,
    SrcNotFound,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub msg: String,
}
impl Error {
    pub fn new<T: Into<String>>(kind: ErrorKind, msg: T) -> Error {
        Error { kind, msg: msg.into() }
    }
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

// Alias for result which
pub type Result<T> = result::Result<T, Error>;

pub(crate) trait FromKind<T> {
    fn from_kind(src: T, kind: ErrorKind) -> Self;
}
impl FromKind<IntError> for Error {
    fn from_kind(src: IntError, kind: ErrorKind) -> Self {
        Error::new(kind, src.msg)
    }
}
impl<T> FromKind<IntResult<T>> for Result<T> {
    fn from_kind(src: IntResult<T>, kind: ErrorKind) -> Self {
        src.map_err(|e| Error::from_kind(e, kind))
    }
}
