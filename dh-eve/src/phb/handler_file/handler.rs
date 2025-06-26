use std::{fmt, fs::File, io::BufReader, path::PathBuf};

use super::{address::Address, error::FromPath};
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

/// Data handler which uses locally stored [Phobos](https://github.com/pyfa-org/Phobos) JSON dump
pub struct PhbFileEdh {
    base_path: PathBuf,
}
impl PhbFileEdh {
    /// Constructs file EVE data handler using provided path.
    ///
    /// Path should point to the top-level folder of a data dump, e.g. `/phobos_en-us` and not
    /// `/phobos_en-us/fsd_built`.
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
    // Entity-specific processing methods
    fn process_binary_types(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "types");
        let data = self.read_json(&addr)?;
        e_data.items = fsd::handle::<PItem, rc::ed::EItem>(data, &addr.get_part_str())?;
        Ok(())
    }
    fn process_binary_groups(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "groups");
        let data = self.read_json(&addr)?;
        e_data.groups = fsd::handle::<PItemGroup, rc::ed::EItemGroup>(data, &addr.get_part_str())?;
        Ok(())
    }
    fn process_binary_typelist(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "typelist");
        let data = self.read_json(&addr)?;
        e_data.item_lists = fsd::handle::<PItemList, rc::ed::EItemList>(data, &addr.get_part_str())?;
        Ok(())
    }
    fn process_binary_dogmaattributes(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "dogmaattributes");
        let data = self.read_json(&addr)?;
        e_data.attrs = fsd::handle::<PAttr, rc::ed::EAttr>(data, &addr.get_part_str())?;
        Ok(())
    }
    fn process_binary_typedogma(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "typedogma");
        let data = self.read_json(&addr)?;
        e_data.item_attrs = fsd::handle::<PItemAttrs, rc::ed::EItemAttr>(data.clone(), &addr.get_part_str())?;
        e_data.item_effects = fsd::handle::<PItemEffects, rc::ed::EItemEffect>(data, &addr.get_part_str())?;
        Ok(())
    }
    fn process_binary_dogmaeffects(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "dogmaeffects");
        let data = self.read_json(&addr)?;
        e_data.effects = fsd::handle::<PEffect, rc::ed::EEffect>(data, &addr.get_part_str())?;
        Ok(())
    }
    fn process_lite_fighterabilities(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_lite", "fighterabilities");
        let data = self.read_json(&addr)?;
        e_data.abils = fsd::handle::<PFighterAbil, rc::ed::EFighterAbil>(data, &addr.get_part_str())?;
        Ok(())
    }
    fn process_lite_fighterabilitiesbytype(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_lite", "fighterabilitiesbytype");
        let data = self.read_json(&addr)?;
        e_data.item_abils = fsd::handle::<PItemFighterAbils, rc::ed::EItemFighterAbil>(data, &addr.get_part_str())?;
        Ok(())
    }
    fn process_lite_dbuffcollections(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_lite", "dbuffcollections");
        let data = self.read_json(&addr)?;
        e_data.buffs = fsd::handle::<PBuff, rc::ed::EBuff>(data, &addr.get_part_str())?;
        Ok(())
    }
    fn process_binary_spacecomponentsbytype(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "spacecomponentsbytype");
        let data = self.read_json(&addr)?;
        e_data.space_comps = fsd::handle::<PItemSpaceComp, rc::ed::EItemSpaceComp>(data, &addr.get_part_str())?;
        Ok(())
    }
    fn process_binary_requiredskillsfortypes(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "requiredskillsfortypes");
        let data = self.read_json(&addr)?;
        e_data.item_srqs = fsd::handle::<PItemSkillMap, rc::ed::EItemSkillReq>(data, &addr.get_part_str())?;
        Ok(())
    }
    fn process_binary_dynamicitemattributes(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "dynamicitemattributes");
        let data = self.read_json(&addr)?;
        e_data.muta_items = fsd::handle::<PMutaItemConvs, rc::ed::EMutaItemConv>(data.clone(), &addr.get_part_str())?;
        e_data.muta_attrs = fsd::handle::<PMutaAttrMods, rc::ed::EMutaAttrMod>(data, &addr.get_part_str())?;
        Ok(())
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
        self.process_binary_types(&mut data)?;
        self.process_binary_groups(&mut data)?;
        self.process_binary_typelist(&mut data)?;
        self.process_binary_dogmaattributes(&mut data)?;
        self.process_binary_typedogma(&mut data)?;
        self.process_binary_dogmaeffects(&mut data)?;
        self.process_lite_fighterabilities(&mut data)?;
        self.process_lite_fighterabilitiesbytype(&mut data)?;
        self.process_lite_dbuffcollections(&mut data)?;
        self.process_binary_spacecomponentsbytype(&mut data)?;
        self.process_binary_requiredskillsfortypes(&mut data)?;
        self.process_binary_dynamicitemattributes(&mut data)?;
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
