use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::result;

use log;
use serde_json::{Map as JsonMap, Value as JsonValue};

use crate::dh;

use super::address::Address;
use super::error::{Error, FromPathErr};

type Result<T> = result::Result<T, Error>;

pub struct Handler {
    base_path: PathBuf,
}

impl Handler {
    pub fn new<P: Into<PathBuf>>(path: P) -> Handler {
        Handler { base_path: path.into() }
    }
    fn read_file(&self, addr: &Address) -> io::Result<Vec<u8>> {
        let full_path = addr.get_full_path(&self.base_path);
        let mut bytes: Vec<u8> = Vec::new();
        File::open(full_path)?.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
    fn read_json(&self, addr: &Address) -> Result<JsonValue> {
        let bytes = self
            .read_file(addr)
            .map_err(|e| Error::from_path_err(e, addr.get_full_str(&self.base_path)))?;
        let data =
            serde_json::from_slice(&bytes).map_err(|e| Error::from_path_err(e, addr.get_full_str(&self.base_path)))?;
        Ok(data)
    }
    fn decompose_map(&self, addr: &Address, json: JsonValue, fields: Vec<&str>) -> Result<dh::Table> {
        let mut rows: Vec<dh::Row> = Vec::new();
        let mut errors: u32 = 0;
        for v in self.check_map(&addr, &json)?.values() {
            match Handler::map_to_datarow(v, &fields) {
                Some(row) => rows.push(row),
                None => {
                    if errors < u32::MAX {
                        errors += 1
                    }
                }
            }
        }
        Ok(dh::Table::new(rows, errors))
    }
    fn check_map<'a>(&self, addr: &Address, json: &'a JsonValue) -> Result<&'a JsonMap<String, JsonValue>> {
        match json.as_object() {
            Some(json) => Ok(json),
            None => Err(Error::new(format!(
                "{} conversion failed: highest-level structure is not a map",
                addr.get_full_str(&self.base_path)
            ))),
        }
    }
    fn map_to_datarow(json_row: &JsonValue, fields: &Vec<&str>) -> Option<dh::Row> {
        let mut row: dh::Row = Vec::new();
        for &field in fields {
            let item = dh::Item::new("name", dh::Value::String(json_row[field].as_str()?.to_string()));
            row.push(item);
        }
        Some(row)
    }
    // fn convert_value(json: &JsonValue) -> Option<DataValue> {
    //     match json {
    //         JsonValue::Null => Some(DataValue::Null),
    //         JsonValue::Bool(&v) => Some(DataValue::Bool(v)),
    //         JsonValue::Number(&v) => Some(DataValue::Null),
    //         JsonValue::String(&v) => Some(DataValue::Null),
    //         JsonValue::Array => Some(DataValue::Null),
    //         JsonValue::Object(&v) => Some(DataValue::Null),
    //     }
    // }
}

impl dh::Handler for Handler {
    fn get_evetypes(&self) -> dh::Result {
        let addr = Address::new("fsd_lite", "evetypes");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        let json = self.read_json(&addr)?;
        let data = self.decompose_map(&addr, json, vec!["typeName"])?;
        Ok(data)
    }
}
