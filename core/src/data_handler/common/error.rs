use std::error::Error;
use std::result::Result;

use super::DataTable;

pub type DataHandlerResult = Result<DataTable, Box<dyn Error>>;
