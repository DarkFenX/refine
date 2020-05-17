use std::error::Error;
use std::fmt;
use std::result::Result;

pub(super) type PhobosHandlerResult<T> = Result<T, PhobosHandlerError>;

#[derive(Debug)]
pub(super) struct PhobosHandlerError {
    pub(super) msg: String,
}

impl Error for PhobosHandlerError {}

impl fmt::Display for PhobosHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
