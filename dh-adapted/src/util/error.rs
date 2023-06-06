use std::{error, fmt};

#[derive(Debug)]
pub enum ErrorKind {
    NoCacheSupport,
    #[cfg(feature = "json")]
    RamJsonReadFailed(String),
    #[cfg(feature = "json")]
    RamJsonDecompFailed(String),
    #[cfg(feature = "json")]
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
            #[cfg(feature = "json")]
            ErrorKind::RamJsonReadFailed(msg) => write!(f, "unable to open cache for reading: {msg}"),
            #[cfg(feature = "json")]
            ErrorKind::RamJsonDecompFailed(msg) => write!(f, "unable to decompress cache: {msg}"),
            #[cfg(feature = "json")]
            ErrorKind::RamJsonParseFailed(msg) => write!(f, "unable to parse cache data: {msg}"),
        }
    }
}
