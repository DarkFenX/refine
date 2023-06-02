use std::{error, fmt};

#[derive(Debug)]
pub enum ErrorKind {
    NoCacheSupport,
    RamJsonReadFailed(String),
    RamJsonDecompFailed(String),
    RamJsonParseFailed(String),
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
            ErrorKind::RamJsonReadFailed(msg) => write!(f, "unable to open cache for reading: {msg}"),
            ErrorKind::RamJsonDecompFailed(msg) => write!(f, "unable to decompress cache: {msg}"),
            ErrorKind::RamJsonParseFailed(msg) => write!(f, "unable to parse cache data: {msg}"),
        }
    }
}
