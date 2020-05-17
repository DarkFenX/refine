use super::DataHandlerResult;
use super::DataRow;

pub trait DataHandler {
    fn get_evetypes(&self) -> DataHandlerResult<Vec<DataRow>>;
}
