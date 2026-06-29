use std::{fmt, fs::File, io::BufReader, path::PathBuf};

use super::{address::Address, error::PhbFileEdhError, parsing::ArrayIter};
use crate::phb::{
    data::{
        PAttr, PBuff, PEffect, PFighterAbil, PItem, PItemDogma, PItemFighterAbils, PItemGroup, PItemList,
        PItemSkillMap, PItemSpaceComp, PMetadata, PMuta,
    },
    parsing::{handle_keymap_one, handle_keymap_two},
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
    fn get_reader(&self, addr: &Address) -> Result<impl std::io::Read, PhbFileEdhError> {
        let full_path = addr.get_full_path(&self.base_path);
        let file = File::open(full_path).map_err(|e| PhbFileEdhError::from_io(e, addr.get_part_str()))?;
        Ok(BufReader::with_capacity(64 * 1024, file))
    }
    // Entity-specific processing methods
    fn process_built_types(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFileEdhError> {
        let addr = Address::new("fsd_built", "types");
        let reader = self.get_reader(&addr)?;
        e_data.items = handle_keymap_one::<PItem, rc::ed::EItem>(reader)
            .map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
        Ok(())
    }
    fn process_built_groups(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFileEdhError> {
        let addr = Address::new("fsd_built", "groups");
        let reader = self.get_reader(&addr)?;
        e_data.groups = handle_keymap_one::<PItemGroup, rc::ed::EItemGroup>(reader)
            .map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
        Ok(())
    }
    fn process_built_typelist(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFileEdhError> {
        let addr = Address::new("fsd_built", "typelist");
        let reader = self.get_reader(&addr)?;
        e_data.item_lists = handle_keymap_one::<PItemList, rc::ed::EItemList>(reader)
            .map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
        Ok(())
    }
    fn process_built_dogmaattributes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFileEdhError> {
        let addr = Address::new("fsd_built", "dogmaattributes");
        let reader = self.get_reader(&addr)?;
        e_data.attrs = handle_keymap_one::<PAttr, rc::ed::EAttr>(reader)
            .map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
        Ok(())
    }
    fn process_built_typedogma(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFileEdhError> {
        let addr = Address::new("fsd_built", "typedogma");
        let reader = self.get_reader(&addr)?;
        (e_data.item_attrs, e_data.item_effects) =
            handle_keymap_two::<PItemDogma, rc::ed::EItemAttr, rc::ed::EItemEffect>(reader)
                .map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
        Ok(())
    }
    fn process_built_dogmaeffects(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFileEdhError> {
        let addr = Address::new("fsd_built", "dogmaeffects");
        let reader = self.get_reader(&addr)?;
        e_data.effects = handle_keymap_one::<PEffect, rc::ed::EEffect>(reader)
            .map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
        Ok(())
    }
    fn process_lite_fighterabilities(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFileEdhError> {
        let addr = Address::new("fsd_lite", "fighterabilities");
        let reader = self.get_reader(&addr)?;
        e_data.abils = handle_keymap_one::<PFighterAbil, rc::ed::EAbil>(reader)
            .map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
        Ok(())
    }
    fn process_lite_fighterabilitiesbytype(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFileEdhError> {
        let addr = Address::new("fsd_lite", "fighterabilitiesbytype");
        let reader = self.get_reader(&addr)?;
        e_data.item_abils = handle_keymap_one::<PItemFighterAbils, rc::ed::EItemAbil>(reader)
            .map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
        Ok(())
    }
    fn process_lite_dbuffcollections(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFileEdhError> {
        let addr = Address::new("fsd_lite", "dbuffcollections");
        let reader = self.get_reader(&addr)?;
        e_data.buffs = handle_keymap_one::<PBuff, rc::ed::EBuff>(reader)
            .map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
        Ok(())
    }
    fn process_built_spacecomponentsbytype(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFileEdhError> {
        let addr = Address::new("fsd_built", "spacecomponentsbytype");
        let reader = self.get_reader(&addr)?;
        e_data.space_comps = handle_keymap_one::<PItemSpaceComp, rc::ed::EItemSpaceComp>(reader)
            .map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
        Ok(())
    }
    fn process_built_requiredskillsfortypes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFileEdhError> {
        let addr = Address::new("fsd_built", "requiredskillsfortypes");
        let reader = self.get_reader(&addr)?;
        e_data.item_srqs = handle_keymap_one::<PItemSkillMap, rc::ed::EItemSkillReq>(reader)
            .map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
        Ok(())
    }
    fn process_built_dynamicitemattributes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFileEdhError> {
        let addr = Address::new("fsd_built", "dynamicitemattributes");
        let reader = self.get_reader(&addr)?;
        (e_data.muta_items, e_data.muta_attrs) =
            handle_keymap_two::<PMuta, rc::ed::EMutaItemConv, rc::ed::EMutaAttrMod>(reader)
                .map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
        Ok(())
    }
}
impl fmt::Debug for PhbFileEdh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhbFileEdh(\"{}\")", self.base_path.to_str().unwrap_or("<error>"))
    }
}
impl rc::ed::EveDataHandler for PhbFileEdh {
    fn get_data(&self) -> Result<rc::ed::EData, Box<dyn std::error::Error>> {
        let mut data = rc::ed::EData::new();
        self.process_built_types(&mut data)?;
        self.process_built_groups(&mut data)?;
        self.process_built_typelist(&mut data)?;
        self.process_built_dogmaattributes(&mut data)?;
        self.process_built_typedogma(&mut data)?;
        self.process_built_dogmaeffects(&mut data)?;
        self.process_lite_fighterabilities(&mut data)?;
        self.process_lite_fighterabilitiesbytype(&mut data)?;
        self.process_lite_dbuffcollections(&mut data)?;
        self.process_built_spacecomponentsbytype(&mut data)?;
        self.process_built_requiredskillsfortypes(&mut data)?;
        self.process_built_dynamicitemattributes(&mut data)?;
        Ok(data)
    }
    fn get_data_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        let addr = Address::new("phobos", "metadata");
        let reader = self.get_reader(&addr)?;
        for metadata_result in ArrayIter::new(reader) {
            let metadata: PMetadata =
                metadata_result.map_err(|e| PhbFileEdhError::from_read_parse(e, addr.get_part_str()))?;
            if metadata.field_name == "client_build" {
                return Ok(metadata.field_value.to_string());
            }
        }
        Err(PhbFileEdhError::NoClientBuild.into())
    }
}
