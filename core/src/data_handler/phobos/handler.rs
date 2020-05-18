use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use log::info;
use serde_json::{Map as JsonMap, Value as JsonValue};

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
        let mut rows: Vec<DataRow> = Vec::new();
        let mut errors: u32 = 0;
        for v in self.check_map(&addr, &json)?.values() {
            match self.map_to_datarow(v) {
                Some(row) => rows.push(row),
                None => errors += 1,
            }
        }
        Ok(DataTable::new(rows, errors))
    }
    fn check_map<'a>(&self, addr: &PhobosAddress, json: &'a JsonValue) -> PhobosResult<&'a JsonMap<String, JsonValue>> {
        match json.as_object() {
            Some(json) => Ok(json),
            None => {
                Err(PhobosHandlerError::new(format!(
                    "{} conversion failed: highest-level structure is not a map",
                    addr.get_full_str(&self.base_path)
                )))
            }
        }
    }
    fn map_to_datarow(&self, json_row: &JsonValue) -> Option<DataRow> {
        let mut row: DataRow = Vec::new();
        let item = DataItem::new("name", DataValue::String(json_row["typeName"].as_str()?.to_string()));
        row.push(item);
        Some(row)
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
