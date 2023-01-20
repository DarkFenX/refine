use std::{
    fmt,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use crate::{
    dh,
    util::{Error, Result},
};

use super::{
    super::{
        data::{
            Attr, Buff, Effect, FighterAbil, Item, ItemAttrs, ItemEffects, ItemFighterAbils, ItemGroup, ItemSkillMap,
            Metadata, MutaAttrMods, MutaItemConvs,
        },
        fsd,
    },
    address::Address,
    error::FromPath,
};

/// A struct for extracting data from [Phobos](https://github.com/pyfa-org/Phobos) JSON dump
pub struct PhbFileDHandler {
    base_path: PathBuf,
}
impl PhbFileDHandler {
    /// Constructs new `PhbFileDHandler` using provided path.
    ///
    /// Path should point to the top-level folder of a data dump, e.g. `/phobos_en-us` and not
    /// `/phobos_en-us/fsd_binary`.
    pub fn new<T: Into<PathBuf>>(path: T) -> PhbFileDHandler {
        PhbFileDHandler { base_path: path.into() }
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
impl fmt::Debug for PhbFileDHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PhbFileDHandler(\"{}\")",
            self.base_path.to_str().unwrap_or("<error>")
        )
    }
}
impl dh::DataHandler for PhbFileDHandler {
    fn get_items(&self) -> dh::Result<dh::Container<dh::Item>> {
        self.process_fsd::<Item, dh::Item>("fsd_binary", "types")
    }
    fn get_item_groups(&self) -> dh::Result<dh::Container<dh::ItemGroup>> {
        self.process_fsd::<ItemGroup, dh::ItemGroup>("fsd_binary", "groups")
    }
    fn get_attrs(&self) -> dh::Result<dh::Container<dh::Attr>> {
        self.process_fsd::<Attr, dh::Attr>("fsd_binary", "dogmaattributes")
    }
    fn get_item_attrs(&self) -> dh::Result<dh::Container<dh::ItemAttr>> {
        self.process_fsd::<ItemAttrs, dh::ItemAttr>("fsd_binary", "typedogma")
    }
    fn get_effects(&self) -> dh::Result<dh::Container<dh::Effect>> {
        self.process_fsd::<Effect, dh::Effect>("fsd_binary", "dogmaeffects")
    }
    fn get_item_effects(&self) -> dh::Result<dh::Container<dh::ItemEffect>> {
        self.process_fsd::<ItemEffects, dh::ItemEffect>("fsd_binary", "typedogma")
    }
    fn get_fighter_abils(&self) -> dh::Result<dh::Container<dh::FighterAbil>> {
        self.process_fsd::<FighterAbil, dh::FighterAbil>("fsd_lite", "fighterabilities")
    }
    fn get_item_fighter_abils(&self) -> dh::Result<dh::Container<dh::ItemFighterAbil>> {
        self.process_fsd::<ItemFighterAbils, dh::ItemFighterAbil>("fsd_lite", "fighterabilitiesbytype")
    }
    fn get_buffs(&self) -> dh::Result<dh::Container<dh::Buff>> {
        self.process_fsd::<Buff, dh::Buff>("fsd_lite", "dbuffcollections")
    }
    fn get_item_skill_reqs(&self) -> dh::Result<dh::Container<dh::ItemSkillReq>> {
        self.process_fsd::<ItemSkillMap, dh::ItemSkillReq>("fsd_binary", "requiredskillsfortypes")
    }
    fn get_muta_item_convs(&self) -> dh::Result<dh::Container<dh::MutaItemConv>> {
        self.process_fsd::<MutaItemConvs, dh::MutaItemConv>("fsd_binary", "dynamicitemattributes")
    }
    fn get_muta_attr_mods(&self) -> dh::Result<dh::Container<dh::MutaAttrMod>> {
        self.process_fsd::<MutaAttrMods, dh::MutaAttrMod>("fsd_binary", "dynamicitemattributes")
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
