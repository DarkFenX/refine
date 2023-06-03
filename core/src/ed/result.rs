use std::error::Error;

/// Alias for `Result` which accepts any error type.
pub type EResult<T> = Result<T, Box<dyn Error>>;
