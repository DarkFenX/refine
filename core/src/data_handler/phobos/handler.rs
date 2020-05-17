use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use log::{error, info};
use serde_json::Value;

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

    fn _read_json(&self, addr: &PhobosAddress) -> PhobosHandlerResult<Value> {
        let bytes = match self._read_file(addr) {
            Ok(bytes) => bytes,
            Err(e) => {
                let err = PhobosHandlerError {
                    msg: format!("{} read failed: {}", addr.get_full_str(&self.base_path), e),
                };
                error!("{}", err);
                return Err(err);
            }
        };
        let data = match serde_json::from_slice(&bytes) {
            Ok(data) => data,
            Err(e) => {
                let err = PhobosHandlerError {
                    msg: format!(
                        "{} decode failed: {}",
                        addr.get_full_str(&self.base_path),
                        e
                    ),
                };
                error!("{}", err);
                return Err(err);
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
        let _json = self._read_json(&addr)?;
        let data = Vec::new();
        Ok(data)
    }
}
