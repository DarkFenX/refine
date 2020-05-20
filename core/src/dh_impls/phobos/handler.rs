use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use log;
use serde_json;

use crate::dh;

use super::address::Address;
use super::data::{EveGroup, EveType, FighterAbil, Metadata, TypeFighterAbil};
use super::error::{Error, FromPath, Result};
use super::fsdlite;

#[derive(Debug)]
pub struct Handler {
    base_path: PathBuf,
}
impl Handler {
    pub fn new<T: Into<PathBuf>>(path: T) -> Handler {
        Handler { base_path: path.into() }
    }
    fn read_file(&self, addr: &Address) -> io::Result<Vec<u8>> {
        let full_path = addr.get_full_path(&self.base_path);
        let mut bytes = Vec::new();
        File::open(full_path)?.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
    fn read_json(&self, addr: &Address) -> Result<serde_json::Value> {
        let bytes = self
            .read_file(addr)
            .map_err(|e| Error::from_path(e, addr.get_part_str()))?;
        let data = serde_json::from_slice(&bytes).map_err(|e| Error::from_path(e, addr.get_part_str()))?;
        Ok(data)
    }
}
impl dh::Handler for Handler {
    fn get_evetypes(&self) -> dh::Result<dh::Container<dh::EveType>> {
        let addr = Address::new("fsd_lite", "evetypes");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        let json = self.read_json(&addr)?;
        fsdlite::handle::<EveType, dh::EveType>(json)
    }
    fn get_evegroups(&self) -> dh::Result<dh::Container<dh::EveGroup>> {
        let addr = Address::new("fsd_lite", "evegroups");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        let json = self.read_json(&addr)?;
        fsdlite::handle::<EveGroup, dh::EveGroup>(json)
    }
    fn get_fighterabils(&self) -> dh::Result<dh::Container<dh::FighterAbil>> {
        let addr = Address::new("fsd_lite", "fighterabilities");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        let json = self.read_json(&addr)?;
        fsdlite::handle::<FighterAbil, dh::FighterAbil>(json)
    }
    fn get_typefighterabils(&self) -> dh::Result<dh::Container<dh::TypeFighterAbil>> {
        let addr = Address::new("fsd_lite", "fighterabilitiesbytype");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        let json = self.read_json(&addr)?;
        fsdlite::handle::<TypeFighterAbil, dh::TypeFighterAbil>(json)
    }
    fn get_version(&self) -> dh::Result<String> {
        let addr = Address::new("phobos", "metadata");
        log::info!("processing {}", addr.get_full_str(&self.base_path));
        let unprocessed = self.read_json(&addr)?;
        let metadatas: Vec<Metadata> =
            serde_json::from_value(unprocessed).map_err(|e| Error::from_path(e, addr.get_part_str()))?;
        let mut version = None;
        for metadata in metadatas {
            if metadata.field_name == "client_build" {
                version = Some(metadata.field_value);
                break;
            }
        }
        match version {
            Some(v) => Ok(v.to_string()),
            None => Err(Error::new("version fetch failed: unable to find client build").into()),
        }
    }
}
