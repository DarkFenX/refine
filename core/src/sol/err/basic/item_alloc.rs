use std::{error, fmt};

#[derive(Debug)]
pub struct ItemAllocError {}
impl ItemAllocError {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
impl error::Error for ItemAllocError {}
impl fmt::Display for ItemAllocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "item ID allocation failed")
    }
}
