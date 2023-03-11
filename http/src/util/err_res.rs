use std::{error, fmt, result};

#[derive(Debug)]
pub(crate) enum ErrorKind {
    SrcNotFound,
    SrcAliasNotAvailable,
    DhInitFailed,
    SrcInitFailed,
}

#[derive(Debug)]
pub(crate) struct Error {
    pub kind: ErrorKind,
    pub msg: String,
}
impl Error {
    pub(crate) fn new<T: Into<String>>(kind: ErrorKind, msg: T) -> Error {
        Error { kind, msg: msg.into() }
    }
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub(crate) type Result<T> = result::Result<T, Error>;
