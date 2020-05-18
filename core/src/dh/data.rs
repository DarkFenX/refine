use crate::defines::{ReeFloat, ReeInt};

#[derive(Debug)]
pub struct Table {
    pub data: Vec<Row>,
    pub failed_rows: u32,
}

impl Table {
    pub fn new(data: Vec<Row>, failed_rows: u32) -> Table {
        Table { data, failed_rows }
    }
}

pub type Row = Vec<Item>;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub value: Value,
}

impl Item {
    pub fn new<P: Into<String>>(name: P, value: Value) -> Item {
        Item {
            name: name.into(),
            value,
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Int(ReeInt),
    Float(ReeFloat),
    String(String),
    Yaml(String),
}
