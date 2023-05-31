use std::{error, fmt, result};

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedFsdTopEntity,
    HttpInvalidBaseUrl(String, String),
    HttpSuffixJoinFailed(String, String),
    HttpSuffixFetchFailed(String, String),
    HttpSuffixParseFailed(String, String),
    FilePathReadFailed(String, String),
    FilePathParseFailed(String, String),
    FileNoClientBuild,
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
            ErrorKind::UnexpectedFsdTopEntity => {
                write!(f, "FSD decomposition failed: highest-level entity is not a map")
            }
            ErrorKind::HttpInvalidBaseUrl(url, msg) => write!(f, "invalid base URL \"{url}\": {msg}"),
            ErrorKind::HttpSuffixJoinFailed(suffix, msg) => {
                write!(f, "{suffix} is failed to be joined to base URL: {msg}")
            }
            ErrorKind::HttpSuffixFetchFailed(suffix, msg) => write!(f, "{suffix} fetching failed: {msg}"),
            ErrorKind::HttpSuffixParseFailed(suffix, msg) => write!(f, "{suffix} parsing failed: {msg}"),
            ErrorKind::FilePathReadFailed(path, msg) => write!(f, "{path} reading failed: {msg}"),
            ErrorKind::FilePathParseFailed(path, msg) => write!(f, "{path} parsing failed: {msg}"),
            ErrorKind::FileNoClientBuild => write!(f, "unable to find client build field"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
