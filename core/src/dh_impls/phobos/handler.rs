use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::result;

use log;
use serde;
use serde_json;

use crate::dh;

use super::address::Address;
use super::data::{EveGroup, EveType};
use super::error::{Error, FromPathErr};

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
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
            .map_err(|e| Error::from_path_err(e, addr.get_part_str()))?;
        let data = serde_json::from_slice(&bytes).map_err(|e| Error::from_path_err(e, addr.get_part_str()))?;
        Ok(data)
    }
    // FSD Lite methods
    fn handle_fsdlite<T, U>(&self, addr: &Address) -> dh::Result<U>
    where
        T: serde::de::DeserializeOwned + Into<U>,
    {
        let unprocessed = self.read_json(&addr)?;
        let decomposed = Handler::decompose_fsdlite(&addr, unprocessed)?;
        Handler::convert_fsdlite::<T, U>(decomposed)
    }
    fn decompose_fsdlite(addr: &Address, json: serde_json::Value) -> Result<Vec<serde_json::Value>> {
        match json {
            serde_json::Value::Object(mut map) => Ok(map.values_mut().map(|v| v.take()).collect()),
            _ => Err(Error::new(format!(
                "{} FSD Lite decomposition failed: highest-level structure is not a map",
                addr.get_part_str()
            ))),
        }
    }
    fn convert_fsdlite<T, U>(decomposed: Vec<serde_json::Value>) -> dh::Result<U>
    where
        T: serde::de::DeserializeOwned + Into<U>,
    {
        let mut data = Vec::new();
        let mut errors: u32 = 0;
        for value in decomposed {
            match serde_json::from_value::<T>(value) {
                Ok(v) => data.push(v.into()),
                Err(_) => errors += 1,
            }
        }
        Ok(dh::Container::new(data, errors))
    }
}
impl dh::Handler for Handler {
    fn get_evetypes(&self) -> dh::Result<dh::EveType> {
        let addr = Address::new("fsd_lite", "evetypes");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        self.handle_fsdlite::<EveType, dh::EveType>(&addr)
    }
    fn get_evegroups(&self) -> dh::Result<dh::EveGroup> {
        let addr = Address::new("fsd_lite", "evegroups");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        self.handle_fsdlite::<EveGroup, dh::EveGroup>(&addr)
    }
}
