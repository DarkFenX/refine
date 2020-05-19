use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::result;

use log;
use serde;
use serde_yaml;

use crate::defines::ReeInt;
use crate::dh;

use super::address::Address;
use super::data::{Assemble, EveGroup, EveType, FsdItem};
use super::error::{Error, FromPath};

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
        let mut bytes = Vec::new();
        File::open(full_path)?.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
    fn read_yaml(&self, addr: &Address) -> Result<serde_yaml::Value> {
        let bytes = self
            .read_file(addr)
            .map_err(|e| Error::from_path(e, addr.get_part_str()))?;
        let data = serde_yaml::from_slice(&bytes).map_err(|e| Error::from_path(e, addr.get_part_str()))?;
        Ok(data)
    }
    // FSD methods
    fn handle_fsd<T, U>(&self, addr: &Address) -> dh::Result<dh::Container<U>>
    where
        T: serde::de::DeserializeOwned + Assemble<U>,
    {
        let unprocessed = self.read_yaml(&addr)?;
        let decomposed = Handler::decompose_fsd(&addr, unprocessed)?;
        Handler::convert_fsd::<T, U>(decomposed)
    }
    fn decompose_fsd(addr: &Address, yaml: serde_yaml::Value) -> Result<Vec<FsdItem>> {
        match yaml {
            serde_yaml::Value::Mapping(map) => Ok(map.into_iter().map(|(k, v)| FsdItem::new(k, v)).collect()),
            _ => Err(Error::new(format!(
                "{} FSD decomposition failed: highest-level structure is not a map",
                addr.get_part_str()
            ))),
        }
    }
    fn convert_fsd<T, U>(decomposed: Vec<FsdItem>) -> dh::Result<dh::Container<U>>
    where
        T: serde::de::DeserializeOwned + Assemble<U>,
    {
        let mut data = Vec::new();
        let mut errors: u32 = 0;
        for fsd_item in decomposed {
            match serde_yaml::from_value::<ReeInt>(fsd_item.id) {
                Ok(id) => match serde_yaml::from_value::<T>(fsd_item.item) {
                    Ok(item) => data.push(item.assemble(id)),
                    Err(_) => {
                        errors += 1;
                        continue;
                    }
                },
                Err(_) => {
                    errors += 1;
                    continue;
                }
            }
        }
        Ok(dh::Container::new(data, errors))
    }
}
impl dh::Handler for Handler {
    fn get_evetypes(&self) -> dh::Result<dh::Container<dh::EveType>> {
        let addr = Address::new("fsd", "typeIDs");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        self.handle_fsd::<EveType, dh::EveType>(&addr)
    }
    fn get_evegroups(&self) -> dh::Result<dh::Container<dh::EveGroup>> {
        let addr = Address::new("fsd", "groupIDs");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        self.handle_fsd::<EveGroup, dh::EveGroup>(&addr)
    }
    fn get_version(&self) -> dh::Result<String> {
        Ok(String::from(""))
    }
}
