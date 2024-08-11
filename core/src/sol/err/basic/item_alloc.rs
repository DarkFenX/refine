#[derive(Debug)]
pub struct ItemAllocError {}
impl ItemAllocError {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
impl std::error::Error for ItemAllocError {}
impl std::fmt::Display for ItemAllocError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "item ID allocation failed")
    }
}
