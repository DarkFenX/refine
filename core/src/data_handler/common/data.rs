use crate::defines::{ReeFloat, ReeInt};

#[derive(Debug)]
pub struct DataTable {
    pub data: Vec<DataRow>,
    pub failed_rows: u32,
}

impl DataTable {
    pub fn new(data: Vec<DataRow>, failed_rows: u32) -> DataTable {
        DataTable { data, failed_rows }
    }
}

pub type DataRow = Vec<DataItem>;

#[derive(Debug)]
pub struct DataItem {
    pub name: String,
    pub value: DataValue,
}

impl DataItem {
    pub fn new<P: Into<String>>(name: P, value: DataValue) -> DataItem {
        DataItem {
            name: name.into(),
            value,
        }
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
