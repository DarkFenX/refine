use std::{error, fmt};

#[derive(Debug)]
pub struct FitAllocError {}
impl FitAllocError {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
impl error::Error for FitAllocError {}
impl fmt::Display for FitAllocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "item ID allocation failed")
    }
}
