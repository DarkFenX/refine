#[derive(Debug)]
pub struct FitAllocError {}
impl FitAllocError {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
impl std::error::Error for FitAllocError {}
impl std::fmt::Display for FitAllocError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item ID allocation failed")
    }
}
