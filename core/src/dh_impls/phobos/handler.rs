use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::result;

use log;
use serde_json;

use crate::dh;

use super::address::Address;
use super::data::EveType;
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
    fn read_json(&self, addr: &Address) -> Result<serde_json::Value> {
        let bytes = self
            .read_file(addr)
            .map_err(|e| Error::from_path_err(e, addr.get_full_str(&self.base_path)))?;
        let data =
            serde_json::from_slice(&bytes).map_err(|e| Error::from_path_err(e, addr.get_full_str(&self.base_path)))?;
        Ok(data)
    }
    fn decompose_fsdlite(&self, addr: &Address, json: serde_json::Value) -> Result<Vec<serde_json::Value>> {
        match json {
            serde_json::Value::Object(mut map) => {
                let mut vals: Vec<serde_json::Value> = Vec::new();
                for val in map.values_mut() {
                    vals.push(val.take());
                }
                Ok(vals)
            }
            _ => Err(Error::new(format!(
                "{} FSD Lite decomposition failed: highest-level structure is not a map",
                addr.get_full_str(&self.base_path)
            ))),
        }
    }
}

impl dh::Handler for Handler {
    fn get_evetypes(&self) -> dh::Result<dh::EveType> {
        let addr = Address::new("fsd_lite", "evetypes");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        let unprocessed = self.read_json(&addr)?;
        let decomposed = self.decompose_fsdlite(&addr, unprocessed)?;
        let mut data = Vec::new();
        let mut errors: u32 = 0;
        for value in decomposed {
            match serde_json::from_value::<EveType>(value) {
                Ok(v) => data.push(v.into()),
                Err(_) => errors += 1,
            }
        }
        Ok(dh::Container::new(data, errors))
    }
}
