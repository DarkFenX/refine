use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use crate::dh;
use crate::dh_impls::phobos::data::DgmTypeEffects;

use super::address::Address;
use super::data::{DgmAttr, DgmBuff, DgmEffect, DgmTypeAttrs, FtrAbil, FtrTypeAbil, InvGroup, InvType, Metadata};
use super::error::{Error, FromPath, Result};
use super::fsd;

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
    fn get_invtypes(&self) -> dh::Result<dh::Container<dh::InvType>> {
        let addr = Address::new("fsd_lite", "evetypes");
        let json = self.read_json(&addr)?;
        fsd::handle::<InvType, dh::InvType>(json)
    }
    fn get_invgroups(&self) -> dh::Result<dh::Container<dh::InvGroup>> {
        let addr = Address::new("fsd_lite", "evegroups");
        let json = self.read_json(&addr)?;
        fsd::handle::<InvGroup, dh::InvGroup>(json)
    }
    fn get_dgmattrs(&self) -> dh::Result<dh::Container<dh::DgmAttr>> {
        let addr = Address::new("fsd_binary", "dogmaattributes");
        let json = self.read_json(&addr)?;
        fsd::handle::<DgmAttr, dh::DgmAttr>(json)
    }
    fn get_dgmtypeattrs(&self) -> dh::Result<dh::Container<dh::DgmTypeAttr>> {
        let addr = Address::new("fsd_binary", "typedogma");
        let json = self.read_json(&addr)?;
        fsd::handle::<DgmTypeAttrs, dh::DgmTypeAttr>(json)
    }
    fn get_dgmeffects(&self) -> dh::Result<dh::Container<dh::DgmEffect>> {
        let addr = Address::new("fsd_binary", "dogmaeffects");
        let json = self.read_json(&addr)?;
        fsd::handle::<DgmEffect, dh::DgmEffect>(json)
    }
    fn get_dgmtypeeffects(&self) -> dh::Result<dh::Container<dh::DgmTypeEffect>> {
        let addr = Address::new("fsd_binary", "typedogma");
        let json = self.read_json(&addr)?;
        fsd::handle::<DgmTypeEffects, dh::DgmTypeEffect>(json)
    }
    fn get_dgmbuffs(&self) -> dh::Result<dh::Container<dh::DgmBuff>> {
        let addr = Address::new("fsd_lite", "dbuffcollections");
        let json = self.read_json(&addr)?;
        fsd::handle::<DgmBuff, dh::DgmBuff>(json)
    }
    fn get_ftrabils(&self) -> dh::Result<dh::Container<dh::FtrAbil>> {
        let addr = Address::new("fsd_lite", "fighterabilities");
        let json = self.read_json(&addr)?;
        fsd::handle::<FtrAbil, dh::FtrAbil>(json)
    }
    fn get_ftrtypeabils(&self) -> dh::Result<dh::Container<dh::FtrTypeAbil>> {
        let addr = Address::new("fsd_lite", "fighterabilitiesbytype");
        let json = self.read_json(&addr)?;
        fsd::handle::<FtrTypeAbil, dh::FtrTypeAbil>(json)
    }
    fn get_version(&self) -> dh::Result<String> {
        let addr = Address::new("phobos", "metadata");
        let unprocessed = self.read_json(&addr)?;
        let metadatas: Vec<Metadata> =
            serde_json::from_value(unprocessed).map_err(|e| Error::from_path(e, addr.get_part_str()))?;
        for metadata in metadatas {
            if metadata.field_name == "client_build" {
                return Ok(metadata.field_value.to_string());
            }
        }
        Err(Error::new("version fetch failed: unable to find client build").into())
    }
}
