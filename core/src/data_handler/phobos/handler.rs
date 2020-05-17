use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use log::info;
use serde_json::Value as JsonValue;

use crate::data_handler::common::{DataHandler, DataHandlerResult, DataItem, DataRow, DataTable, DataValue};

use super::address::PhobosAddress;
use super::error::{FromPathErr, PhobosHandlerError};

type PhobosResult<T> = Result<T, PhobosHandlerError>;

pub struct PhobosDataHandler {
    base_path: PathBuf,
}

impl PhobosDataHandler {
    pub fn new<P: Into<PathBuf>>(path: P) -> PhobosDataHandler {
        PhobosDataHandler { base_path: path.into() }
    }
    fn read_file(&self, addr: &PhobosAddress) -> io::Result<Vec<u8>> {
        let full_path = addr.get_full_path(&self.base_path);
        let mut bytes: Vec<u8> = Vec::new();
        File::open(full_path)?.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
    fn read_json(&self, addr: &PhobosAddress) -> PhobosResult<JsonValue> {
        let bytes = self
            .read_file(addr)
            .map_err(|e| PhobosHandlerError::from_path_err(e, addr.get_full_str(&self.base_path)))?;
        let data = serde_json::from_slice(&bytes)
            .map_err(|e| PhobosHandlerError::from_path_err(e, addr.get_full_str(&self.base_path)))?;
        Ok(data)
    }
    fn decompose_map(&self, addr: &PhobosAddress, json: JsonValue) -> PhobosResult<DataTable> {
        let mut data: DataTable = Vec::new();
        match json.as_object() {
            Some(json) => {
                for v in json.values() {
                    let datarow = self.map_to_datarow(v).unwrap();
                    data.push(datarow);
                }
            },
            None => return Err(PhobosHandlerError::new(format!(
                "{} conversion failed: highest-level structure is not a map",
                addr.get_full_str(&self.base_path)
            ))),
        }
        Ok(data)
    }
    fn map_to_datarow(&self, json_row: &JsonValue) -> PhobosResult<DataRow> {
        let mut row: DataRow = Vec::new();
        let item = DataItem::new("name", DataValue::String(json_row["typeName"].as_str().unwrap().to_string()));
        row.push(item);
        Ok(row)
    }

}

impl DataHandler for PhobosDataHandler {
    fn get_evetypes(&self) -> DataHandlerResult {
        let addr = PhobosAddress::new("fsd_lite", "evetypes");
        info!("processing {}", addr.get_full_str(&self.base_path));
        let json = self.read_json(&addr)?;
        let data = self.decompose_map(&addr, json)?;
        Ok(data)
    }
}
