use super::data::DataRow;

pub trait DataHandler {
    fn get_evetypes(&self) -> Vec<DataRow>;
}
