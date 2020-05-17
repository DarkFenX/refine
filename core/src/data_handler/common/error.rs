use std::error::Error;
use std::result::Result;

pub type DataHandlerResult<T> = Result<T, Box<dyn Error>>;
