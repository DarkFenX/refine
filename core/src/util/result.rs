use std::result;

use super::error::Error;

pub type Result<T> = result::Result<T, Error>;
