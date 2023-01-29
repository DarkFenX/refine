use std::{error, fmt};

#[derive(Debug)]
pub(crate) struct IntError {
    pub msg: String,
}
impl IntError {
    pub(crate) fn new<T: Into<String>>(msg: T) -> IntError {
        IntError { msg: msg.into() }
    }
}
impl error::Error for IntError {}
impl fmt::Display for IntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub(crate) type IntResult<T> = Result<T, IntError>;
