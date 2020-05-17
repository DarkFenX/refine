use serde_json;
use std::error::Error;
use std::fmt;
use std::io;
use std::result::Result;

pub(super) type PhobosHandlerResult<T> = Result<T, PhobosHandlerError>;

pub(super) trait FromPathErr<T> {
    fn from_path_err<P: Into<String>>(err: T, path: P) -> Self;
}

#[derive(Debug)]
pub(super) struct PhobosHandlerError {
    pub(super) msg: String,
}

impl PhobosHandlerError {
    pub fn new<P: Into<String>>(msg: P) -> PhobosHandlerError {
        PhobosHandlerError { msg: msg.into() }
    }
}

impl Error for PhobosHandlerError {}

impl fmt::Display for PhobosHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl FromPathErr<io::Error> for PhobosHandlerError {
    fn from_path_err<P: Into<String>>(err: io::Error, path: P) -> Self {
        PhobosHandlerError::new(format!("{} reading failed: {}", path.into(), err))
    }
}

impl FromPathErr<serde_json::Error> for PhobosHandlerError {
    fn from_path_err<P: Into<String>>(err: serde_json::Error, path: P) -> Self {
        PhobosHandlerError::new(format!("{} parsing failed: {}", path.into(), err))
    }
}
