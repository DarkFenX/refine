use crate::defines::{ReeFloat, ReeInt};

pub type DataTable = Vec<DataRow>;

pub type DataRow = Vec<DataItem>;

#[derive(Debug)]
pub struct DataItem {
    pub name: String,
    pub value: DataValue,
}

impl DataItem {
    pub fn new<P: Into<String>>(name: P, value: DataValue) -> DataItem {
        DataItem{name: name.into(), value}
    }
}

#[derive(Debug)]
pub enum DataValue {
    Null,
    Bool(bool),
    Int(ReeInt),
    Float(ReeFloat),
    String(String),
    Yaml(String),
}
