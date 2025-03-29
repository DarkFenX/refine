use std::{fmt, fs::File, io::BufReader, path::PathBuf};

use crate::{
    phb::{
        data::{
            PAttr, PBuff, PEffect, PFighterAbil, PItem, PItemAttrs, PItemEffects, PItemFighterAbils, PItemGroup,
            PItemList, PItemSkillMap, PItemSpaceComp, PMetadata, PMutaAttrMods, PMutaItemConvs,
        },
        fsd,
    },
    util::Error,
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
    fn read_json(&self, addr: &Address) -> Result<serde_json::Value, Error> {
        let full_path = addr.get_full_path(&self.base_path);
        let file = File::open(full_path).map_err(|e| Error::from_path(e, &addr.get_part_str()))?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader).map_err(|e| Error::from_path(e, &addr.get_part_str()))?;
        Ok(data)
    }
    fn process_fsd<T, U>(&self, folder: &'static str, file: &'static str) -> rc::ed::EResult<rc::ed::EDataCont<U>>
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
impl rc::ed::EveDataHandler for PhbFileEdh {
    fn get_items(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItem>> {
        self.process_fsd::<PItem, rc::ed::EItem>("fsd_binary", "types")
    }
    fn get_item_groups(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemGroup>> {
        self.process_fsd::<PItemGroup, rc::ed::EItemGroup>("fsd_binary", "groups")
    }
    fn get_item_lists(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemList>> {
        self.process_fsd::<PItemList, rc::ed::EItemList>("fsd_binary", "typelist")
    }
    fn get_attrs(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EAttr>> {
        self.process_fsd::<PAttr, rc::ed::EAttr>("fsd_binary", "dogmaattributes")
    }
    fn get_item_attrs(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemAttr>> {
        self.process_fsd::<PItemAttrs, rc::ed::EItemAttr>("fsd_binary", "typedogma")
    }
    fn get_effects(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EEffect>> {
        self.process_fsd::<PEffect, rc::ed::EEffect>("fsd_binary", "dogmaeffects")
    }
    fn get_item_effects(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemEffect>> {
        self.process_fsd::<PItemEffects, rc::ed::EItemEffect>("fsd_binary", "typedogma")
    }
    fn get_fighter_abils(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EFighterAbil>> {
        self.process_fsd::<PFighterAbil, rc::ed::EFighterAbil>("fsd_lite", "fighterabilities")
    }
    fn get_item_fighter_abils(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemFighterAbil>> {
        self.process_fsd::<PItemFighterAbils, rc::ed::EItemFighterAbil>("fsd_lite", "fighterabilitiesbytype")
    }
    fn get_buffs(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EBuff>> {
        self.process_fsd::<PBuff, rc::ed::EBuff>("fsd_lite", "dbuffcollections")
    }
    fn get_space_comps(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemSpaceComp>> {
        self.process_fsd::<PItemSpaceComp, rc::ed::EItemSpaceComp>("fsd_binary", "spacecomponentsbytype")
    }
    fn get_item_skill_reqs(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EItemSkillReq>> {
        self.process_fsd::<PItemSkillMap, rc::ed::EItemSkillReq>("fsd_binary", "requiredskillsfortypes")
    }
    fn get_muta_item_convs(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EMutaItemConv>> {
        self.process_fsd::<PMutaItemConvs, rc::ed::EMutaItemConv>("fsd_binary", "dynamicitemattributes")
    }
    fn get_muta_attr_mods(&self) -> rc::ed::EResult<rc::ed::EDataCont<rc::ed::EMutaAttrMod>> {
        self.process_fsd::<PMutaAttrMods, rc::ed::EMutaAttrMod>("fsd_binary", "dynamicitemattributes")
    }
    fn get_data_version(&self) -> rc::ed::EResult<String> {
        // Uses `client_build` value of the metadata file as version.
        let addr = Address::new("phobos", "metadata");
        let unprocessed = self.read_json(&addr)?;
        let metadatas: Vec<PMetadata> =
            serde_json::from_value(unprocessed).map_err(|e| Error::from_path(e, &addr.get_part_str()))?;
        for metadata in metadatas {
            if metadata.field_name == "client_build" {
                return Ok(metadata.field_value.to_string());
            }
        }
        Err(Error::PhbFileNoClientBuild.into())
    }
}
