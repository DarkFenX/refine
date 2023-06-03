/// Convenience type to pass data and accumulated warnings to the caller.
#[derive(Debug)]
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
    /// Make a new container out of passed data.
    pub fn new_with_data(data: Vec<T>, warns: Vec<String>) -> EDataCont<T> {
        EDataCont { data, warns }
    }
}
