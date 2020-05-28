use std::result;

use super::Error;

pub type Result<T> = result::Result<T, Error>;
