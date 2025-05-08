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
    fn get_data(&self) -> rc::ed::EResult<rc::ed::EData> {
        let mut data = rc::ed::EData::new();
        data.items = self.process_fsd::<PItem, rc::ed::EItem>("fsd_binary", "types")?;
        data.groups = self.process_fsd::<PItemGroup, rc::ed::EItemGroup>("fsd_binary", "groups")?;
        data.item_lists = self.process_fsd::<PItemList, rc::ed::EItemList>("fsd_binary", "typelist")?;
        data.attrs = self.process_fsd::<PAttr, rc::ed::EAttr>("fsd_binary", "dogmaattributes")?;
        data.item_attrs = self.process_fsd::<PItemAttrs, rc::ed::EItemAttr>("fsd_binary", "typedogma")?;
        data.effects = self.process_fsd::<PEffect, rc::ed::EEffect>("fsd_binary", "dogmaeffects")?;
        data.item_effects = self.process_fsd::<PItemEffects, rc::ed::EItemEffect>("fsd_binary", "typedogma")?;
        data.abils = self.process_fsd::<PFighterAbil, rc::ed::EFighterAbil>("fsd_lite", "fighterabilities")?;
        data.item_abils =
            self.process_fsd::<PItemFighterAbils, rc::ed::EItemFighterAbil>("fsd_lite", "fighterabilitiesbytype")?;
        data.buffs = self.process_fsd::<PBuff, rc::ed::EBuff>("fsd_lite", "dbuffcollections")?;
        data.space_comps =
            self.process_fsd::<PItemSpaceComp, rc::ed::EItemSpaceComp>("fsd_binary", "spacecomponentsbytype")?;
        data.item_srqs =
            self.process_fsd::<PItemSkillMap, rc::ed::EItemSkillReq>("fsd_binary", "requiredskillsfortypes")?;
        data.muta_items =
            self.process_fsd::<PMutaItemConvs, rc::ed::EMutaItemConv>("fsd_binary", "dynamicitemattributes")?;
        data.muta_attrs =
            self.process_fsd::<PMutaAttrMods, rc::ed::EMutaAttrMod>("fsd_binary", "dynamicitemattributes")?;
        Ok(data)
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
