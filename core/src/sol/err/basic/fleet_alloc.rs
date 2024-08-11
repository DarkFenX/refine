#[derive(Debug)]
pub struct FleetAllocError {}
impl FleetAllocError {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
impl std::error::Error for FleetAllocError {}
impl std::fmt::Display for FleetAllocError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fleet ID allocation failed")
    }
}
