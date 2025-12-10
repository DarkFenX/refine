use std::error::Error;

pub type EResult<T> = Result<T, Box<dyn Error>>;
