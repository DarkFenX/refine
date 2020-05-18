use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::result;

use log;
use serde_json::Value;

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
    fn read_json(&self, addr: &Address) -> Result<Value> {
        let bytes = self
            .read_file(addr)
            .map_err(|e| Error::from_path_err(e, addr.get_full_str(&self.base_path)))?;
        let data =
            serde_json::from_slice(&bytes).map_err(|e| Error::from_path_err(e, addr.get_full_str(&self.base_path)))?;
        Ok(data)
    }
    fn decompose_fsdlite<'a>(&self, addr: &Address, json: &'a Value) -> Result<Vec<&'a Value>> {
        match json.as_object() {
            Some(json) => {
                let mut vals = Vec::new();
                for val in json.values() {
                    vals.push(val);
                }
                Ok(vals)
            }
            None => Err(Error::new(format!(
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
        let _decomposed = self.decompose_fsdlite(&addr, &unprocessed)?;
        let tmp_vec: Vec<dh::EveType> = Vec::new();
        let tmp_cont = dh::Container::new(tmp_vec, 0);
        Ok(tmp_cont)
    }
}
