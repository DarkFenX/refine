use std::{error, fmt};

#[derive(Debug)]
pub(crate) struct IntError {
    pub msg: String,
}
impl IntError {
    pub(crate) fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl error::Error for IntError {}
impl fmt::Display for IntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub(crate) type IntResult<T> = Result<T, IntError>;
