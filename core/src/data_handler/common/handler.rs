use super::DataTable;
use std::error::Error;

pub type DataHandlerResult = Result<DataTable, Box<dyn Error>>;

pub trait DataHandler {
    fn get_evetypes(&self) -> DataHandlerResult;
}
