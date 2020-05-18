use super::data::Table;
use std::error;
use std::result;

pub type Result = result::Result<Table, Box<dyn error::Error>>;

pub trait Handler {
    fn get_evetypes(&self) -> Result;
}
