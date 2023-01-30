use std::{error, result};

/// Alias for `Result` which accepts any error type.
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
