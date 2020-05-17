use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use log::info;
use serde_json::Value as JsonValue;

use crate::data_handler::common::{DataHandler, DataHandlerResult, DataRow};

use super::address::PhobosAddress;
use super::error::{PhobosHandlerError, PhobosHandlerResult};

pub struct PhobosDataHandler {
    base_path: PathBuf,
}

impl PhobosDataHandler {
    pub fn new<P: Into<PathBuf>>(path: P) -> PhobosDataHandler {
        PhobosDataHandler {
            base_path: path.into(),
        }
    }
    fn _read_file(&self, addr: &PhobosAddress) -> io::Result<Vec<u8>> {
        let full_path = addr.get_full_path(&self.base_path);
        let mut bytes: Vec<u8> = Vec::new();
        File::open(full_path)?.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    fn _read_json(&self, addr: &PhobosAddress) -> PhobosHandlerResult<JsonValue> {
        let bytes = match self._read_file(addr) {
            Ok(bytes) => bytes,
            Err(e) => {
                return Err(PhobosHandlerError::new(format!(
                    "{} read failed: {}",
                    addr.get_full_str(&self.base_path),
                    e
                )))
            }
        };
        let data = match serde_json::from_slice(&bytes) {
            Ok(data) => data,
            Err(e) => {
                return Err(PhobosHandlerError::new(format!(
                    "{} parsing failed: {}",
                    addr.get_full_str(&self.base_path),
                    e
                )))
            }
        };
        Ok(data)
    }
}

impl DataHandler for PhobosDataHandler {
    fn get_evetypes(&self) -> DataHandlerResult<Vec<DataRow>> {
        let addr = PhobosAddress {
            folder: "fsd_lite",
            file: "evetypes",
        };
        info!("processing {}", addr.get_full_str(&self.base_path));
        let json = self._read_json(&addr)?;
        let data = Vec::new();
        match json.as_array() {
            Some(json) => {}
            None => {
                let err = PhobosHandlerError::new(format!(
                    "{} conversion failed: highest-level structure is not a mapping",
                    addr.get_full_str(&self.base_path)
                ));
                return Err(err.into());
            }
        }
        Ok(data)
    }
}
