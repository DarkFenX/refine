/// Convenience type to pass data and accumulated warnings to the caller.
pub struct EDataCont<T> {
    /// Vector with actual data.
    pub data: Vec<T>,
    /// Vector with strings which represent warnings encountered during data generation.
    pub warns: Vec<String>,
}
impl<T> EDataCont<T> {
    /// Make a new empty container.
    pub fn new() -> EDataCont<T> {
        EDataCont {
            data: Vec::new(),
            warns: Vec::new(),
        }
    }
}
impl<T> Default for EDataCont<T> {
    fn default() -> Self {
        Self::new()
    }
}
