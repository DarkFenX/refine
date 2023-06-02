use std::{fmt, fs::File, io::BufReader, path::PathBuf};

use crate::{
    data::{
        Attr, Buff, Effect, FighterAbil, Item, ItemAttrs, ItemEffects, ItemFighterAbils, ItemGroup, ItemSkillMap,
        Metadata, MutaAttrMods, MutaItemConvs,
    },
    fsd,
    util::{Error, ErrorKind, Result},
};

use super::{address::Address, error::FromPath};

/// Data handler which uses locally stored [Phobos](https://github.com/pyfa-org/Phobos) JSON dump
pub struct PhbFileEdh {
    base_path: PathBuf,
}
impl PhbFileEdh {
    /// Constructs file EVE data handler using provided path.
    ///
    /// Path should point to the top-level folder of a data dump, e.g. `/phobos_en-us` and not
    /// `/phobos_en-us/fsd_binary`.
    pub fn new(path: PathBuf) -> Self {
        Self { base_path: path }
    }
    fn read_json(&self, addr: &Address) -> Result<serde_json::Value> {
        let full_path = addr.get_full_path(&self.base_path);
        let file = File::open(full_path).map_err(|e| Error::from_path(e, &addr.get_part_str()))?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader).map_err(|e| Error::from_path(e, &addr.get_part_str()))?;
        Ok(data)
    }
    fn process_fsd<T, U>(&self, folder: &'static str, file: &'static str) -> rc::edh::Result<rc::edh::Container<U>>
    where
        T: serde::de::DeserializeOwned + fsd::FsdMerge<U>,
    {
        let addr = Address::new(folder, file);
        let json = self.read_json(&addr)?;
        fsd::handle::<T, U>(json, &addr.get_part_str())
    }
}
impl fmt::Debug for PhbFileEdh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhbFileEdh(\"{}\")", self.base_path.to_str().unwrap_or("<error>"))
    }
}
impl rc::edh::EveDataHandler for PhbFileEdh {
    /// Get item types.
    fn get_items(&self) -> rc::edh::Result<rc::edh::Container<rc::edt::EItem>> {
        self.process_fsd::<Item, rc::edt::EItem>("fsd_binary", "types")
    }
    /// Get item groups.
    fn get_item_groups(&self) -> rc::edh::Result<rc::edh::Container<rc::edt::EItemGroup>> {
        self.process_fsd::<ItemGroup, rc::edt::EItemGroup>("fsd_binary", "groups")
    }
    /// Get dogma attributes.
    fn get_attrs(&self) -> rc::edh::Result<rc::edh::Container<rc::edt::EAttr>> {
        self.process_fsd::<Attr, rc::edt::EAttr>("fsd_binary", "dogmaattributes")
    }
    /// Get an m:n mapping between item types and dogma attributes.
    fn get_item_attrs(&self) -> rc::edh::Result<rc::edh::Container<rc::edt::EItemAttr>> {
        self.process_fsd::<ItemAttrs, rc::edt::EItemAttr>("fsd_binary", "typedogma")
    }
    /// Get dogma effects.
    fn get_effects(&self) -> rc::edh::Result<rc::edh::Container<rc::edt::EEffect>> {
        self.process_fsd::<Effect, rc::edt::EEffect>("fsd_binary", "dogmaeffects")
    }
    /// Get an m:n mapping between item types and dogma effects.
    fn get_item_effects(&self) -> rc::edh::Result<rc::edh::Container<rc::edt::EItemEffect>> {
        self.process_fsd::<ItemEffects, rc::edt::EItemEffect>("fsd_binary", "typedogma")
    }
    /// Get fighter abilities.
    fn get_fighter_abils(&self) -> rc::edh::Result<rc::edh::Container<rc::edt::EFighterAbil>> {
        self.process_fsd::<FighterAbil, rc::edt::EFighterAbil>("fsd_lite", "fighterabilities")
    }
    /// Get an m:n mapping between item types and fighter abilities.
    fn get_item_fighter_abils(&self) -> rc::edh::Result<rc::edh::Container<rc::edt::EItemFighterAbil>> {
        self.process_fsd::<ItemFighterAbils, rc::edt::EItemFighterAbil>("fsd_lite", "fighterabilitiesbytype")
    }
    /// Get dogma buffs.
    fn get_buffs(&self) -> rc::edh::Result<rc::edh::Container<rc::edt::EBuff>> {
        self.process_fsd::<Buff, rc::edt::EBuff>("fsd_lite", "dbuffcollections")
    }
    /// Get item skill requirements.
    fn get_item_skill_reqs(&self) -> rc::edh::Result<rc::edh::Container<rc::edt::EItemSkillReq>> {
        self.process_fsd::<ItemSkillMap, rc::edt::EItemSkillReq>("fsd_binary", "requiredskillsfortypes")
    }
    /// Get mutaplasmid item conversions.
    fn get_muta_item_convs(&self) -> rc::edh::Result<rc::edh::Container<rc::edt::EMutaItemConv>> {
        self.process_fsd::<MutaItemConvs, rc::edt::EMutaItemConv>("fsd_binary", "dynamicitemattributes")
    }
    /// Get mutaplasmid item modifications.
    fn get_muta_attr_mods(&self) -> rc::edh::Result<rc::edh::Container<rc::edt::EMutaAttrMod>> {
        self.process_fsd::<MutaAttrMods, rc::edt::EMutaAttrMod>("fsd_binary", "dynamicitemattributes")
    }
    /// Get version of the data.
    ///
    /// Uses `client_build` value of metadata file as version.
    fn get_version(&self) -> rc::edh::Result<String> {
        let addr = Address::new("phobos", "metadata");
        let unprocessed = self.read_json(&addr)?;
        let metadatas: Vec<Metadata> =
            serde_json::from_value(unprocessed).map_err(|e| Error::from_path(e, &addr.get_part_str()))?;
        for metadata in metadatas {
            if metadata.field_name == "client_build" {
                return Ok(metadata.field_value.to_string());
            }
        }
        Err(Error::new(ErrorKind::FileNoClientBuild).into())
    }
}
