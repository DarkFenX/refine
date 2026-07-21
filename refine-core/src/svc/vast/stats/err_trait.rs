pub trait StatError: std::error::Error {
    /// Fatal errors will be raised for this item for this stat regardless of passed parameters.
    fn is_fatal(&self) -> bool;
}

impl StatError for ! {
    fn is_fatal(&self) -> bool {
        false
    }
}
