use std::{error, fmt};

#[derive(Debug)]
pub(crate) struct DebugError {}
impl DebugError {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
impl error::Error for DebugError {}
impl fmt::Display for DebugError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "debug error")
    }
}

pub(crate) type DebugResult = Result<(), DebugError>;
