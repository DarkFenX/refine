use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use crate::dh;

use super::address::Address;
use super::data::{
    DgmAttr, DgmBuff, DgmEffect, DgmMutaAttrs, DgmMutaTypes, DgmTypeAttrs, DgmTypeEffects, FtrAbil, FtrTypeAbil,
    InvGroup, InvType, Metadata, SkillReq,
};
use super::error::{Error, FromPath, Result};
use super::fsd;

pub struct PhobosHandler {
    base_path: PathBuf,
}
impl PhobosHandler {
    pub fn new<T: Into<PathBuf>>(path: T) -> PhobosHandler {
        PhobosHandler { base_path: path.into() }
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
    fn process_fsd<T, U>(&self, folder: &'static str, file: &'static str) -> dh::Result<dh::Container<U>>
    where
        T: serde::de::DeserializeOwned + fsd::FsdMerge<U>,
    {
        let addr = Address::new(folder, file);
        let json = self.read_json(&addr)?;
        fsd::handle::<T, U>(json)
    }
}
impl fmt::Debug for PhobosHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhobosHandler(\"{}\")", self.base_path.to_str().unwrap_or("<error>"))
    }
}
impl dh::DataHandler for PhobosHandler {
    fn get_invtypes(&self) -> dh::Result<dh::Container<dh::InvType>> {
        self.process_fsd::<InvType, dh::InvType>("fsd_lite", "evetypes")
    }
    fn get_invgroups(&self) -> dh::Result<dh::Container<dh::InvGroup>> {
        self.process_fsd::<InvGroup, dh::InvGroup>("fsd_lite", "evegroups")
    }
    fn get_dgmattrs(&self) -> dh::Result<dh::Container<dh::DgmAttr>> {
        self.process_fsd::<DgmAttr, dh::DgmAttr>("fsd_binary", "dogmaattributes")
    }
    fn get_dgmtypeattrs(&self) -> dh::Result<dh::Container<dh::DgmTypeAttr>> {
        self.process_fsd::<DgmTypeAttrs, dh::DgmTypeAttr>("fsd_binary", "typedogma")
    }
    fn get_dgmeffects(&self) -> dh::Result<dh::Container<dh::DgmEffect>> {
        self.process_fsd::<DgmEffect, dh::DgmEffect>("fsd_binary", "dogmaeffects")
    }
    fn get_dgmtypeeffects(&self) -> dh::Result<dh::Container<dh::DgmTypeEffect>> {
        self.process_fsd::<DgmTypeEffects, dh::DgmTypeEffect>("fsd_binary", "typedogma")
    }
    fn get_dgmmutatypes(&self) -> dh::Result<dh::Container<dh::DgmMutaType>> {
        self.process_fsd::<DgmMutaTypes, dh::DgmMutaType>("fsd_binary", "dynamicitemattributes")
    }
    fn get_dgmmutaattrs(&self) -> dh::Result<dh::Container<dh::DgmMutaAttr>> {
        self.process_fsd::<DgmMutaAttrs, dh::DgmMutaAttr>("fsd_binary", "dynamicitemattributes")
    }
    fn get_dgmbuffs(&self) -> dh::Result<dh::Container<dh::DgmBuff>> {
        self.process_fsd::<DgmBuff, dh::DgmBuff>("fsd_lite", "dbuffcollections")
    }
    fn get_ftrabils(&self) -> dh::Result<dh::Container<dh::FtrAbil>> {
        self.process_fsd::<FtrAbil, dh::FtrAbil>("fsd_lite", "fighterabilities")
    }
    fn get_ftrtypeabils(&self) -> dh::Result<dh::Container<dh::FtrTypeAbil>> {
        self.process_fsd::<FtrTypeAbil, dh::FtrTypeAbil>("fsd_lite", "fighterabilitiesbytype")
    }
    fn get_skillreqs(&self) -> dh::Result<dh::Container<dh::SkillReq>> {
        self.process_fsd::<SkillReq, dh::SkillReq>("fsd_binary", "requiredskillsfortypes")
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
        Err(Error::new("unable to find client build").into())
    }
}
