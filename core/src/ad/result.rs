use std::error::Error;

pub type AResult<T> = Result<T, Box<dyn Error>>;
