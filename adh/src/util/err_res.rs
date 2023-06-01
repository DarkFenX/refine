use std::{error, fmt, result};

#[derive(Debug)]
pub enum ErrorKind {
    NoCacheSupport,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}
impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::NoCacheSupport => write!(f, "handler does not support cache"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
