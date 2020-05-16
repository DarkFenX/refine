use crate::defines::{ReeFloat, ReeInt};

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
