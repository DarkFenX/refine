use crate::defines::{ReeFloat, ReeInt};

pub trait DataHandler {
    fn get_evetypes(&self) -> Vec<DataRow>;
}

pub struct DataRow {
    pub name: String,
    pub value: DataValue,
}

pub enum DataValue {
    Null,
    Bool(bool),
    Int(ReeInt),
    Float(ReeFloat),
    String(String),
    Yaml(String),
}
