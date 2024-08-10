use std::{error, fmt};

#[derive(Debug)]
pub struct FleetAllocError {}
impl FleetAllocError {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
impl error::Error for FleetAllocError {}
impl fmt::Display for FleetAllocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fleet ID allocation failed")
    }
}
