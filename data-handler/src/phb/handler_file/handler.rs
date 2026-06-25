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
    fn get_reader(&self, addr: &Address) -> Result<impl std::io::Read, Error> {
        let full_path = addr.get_full_path(&self.base_path);
        let file = File::open(full_path).map_err(|e| Error::from_path(e, &addr.get_part_str()))?;
        Ok(BufReader::with_capacity(64 * 1024, file))
    }
    // Entity-specific processing methods
    fn process_built_types(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "types");
        let reader = self.get_reader(&addr)?;
        e_data.items = fsd::handle_one::<PItem, rc::ed::EItem>(reader, &addr.get_part_str())?;
        Ok(())
    }
    fn process_built_groups(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "groups");
        let reader = self.get_reader(&addr)?;
        e_data.groups = fsd::handle_one::<PItemGroup, rc::ed::EItemGroup>(reader, &addr.get_part_str())?;
        Ok(())
    }
    fn process_built_typelist(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "typelist");
        let reader = self.get_reader(&addr)?;
        e_data.item_lists = fsd::handle_one::<PItemList, rc::ed::EItemList>(reader, &addr.get_part_str())?;
        Ok(())
    }
    fn process_built_dogmaattributes(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "dogmaattributes");
        let reader = self.get_reader(&addr)?;
        e_data.attrs = fsd::handle_one::<PAttr, rc::ed::EAttr>(reader, &addr.get_part_str())?;
        Ok(())
    }
    fn process_built_typedogma(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "typedogma");
        let reader = self.get_reader(&addr)?;
        e_data.item_attrs = fsd::handle_one::<PItemAttrs, rc::ed::EItemAttr>(reader, &addr.get_part_str())?;
        let reader = self.get_reader(&addr)?;
        e_data.item_effects = fsd::handle_one::<PItemEffects, rc::ed::EItemEffect>(reader, &addr.get_part_str())?;
        Ok(())
    }
    fn process_built_dogmaeffects(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "dogmaeffects");
        let reader = self.get_reader(&addr)?;
        e_data.effects = fsd::handle_one::<PEffect, rc::ed::EEffect>(reader, &addr.get_part_str())?;
        Ok(())
    }
    fn process_lite_fighterabilities(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_lite", "fighterabilities");
        let reader = self.get_reader(&addr)?;
        e_data.abils = fsd::handle_one::<PFighterAbil, rc::ed::EAbil>(reader, &addr.get_part_str())?;
        Ok(())
    }
    fn process_lite_fighterabilitiesbytype(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_lite", "fighterabilitiesbytype");
        let reader = self.get_reader(&addr)?;
        e_data.item_abils = fsd::handle_one::<PItemFighterAbils, rc::ed::EItemAbil>(reader, &addr.get_part_str())?;
        Ok(())
    }
    fn process_lite_dbuffcollections(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_lite", "dbuffcollections");
        let reader = self.get_reader(&addr)?;
        e_data.buffs = fsd::handle_one::<PBuff, rc::ed::EBuff>(reader, &addr.get_part_str())?;
        Ok(())
    }
    fn process_built_spacecomponentsbytype(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "spacecomponentsbytype");
        let reader = self.get_reader(&addr)?;
        e_data.space_comps = fsd::handle_one::<PItemSpaceComp, rc::ed::EItemSpaceComp>(reader, &addr.get_part_str())?;
        Ok(())
    }
    fn process_built_requiredskillsfortypes(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "requiredskillsfortypes");
        let reader = self.get_reader(&addr)?;
        e_data.item_srqs = fsd::handle_one::<PItemSkillMap, rc::ed::EItemSkillReq>(reader, &addr.get_part_str())?;
        Ok(())
    }
    fn process_built_dynamicitemattributes(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let addr = Address::new("fsd_built", "dynamicitemattributes");
        let reader = self.get_reader(&addr)?;
        e_data.muta_items = fsd::handle_one::<PMutaItemConvs, rc::ed::EMutaItemConv>(reader, &addr.get_part_str())?;
        let reader = self.get_reader(&addr)?;
        e_data.muta_attrs = fsd::handle_one::<PMutaAttrMods, rc::ed::EMutaAttrMod>(reader, &addr.get_part_str())?;
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
    fn get_data_version(&self) -> rc::ed::EResult<String> {
        // Uses `client_build` value of the metadata file as version.
        // let addr = Address::new("phobos", "metadata");
        // let unprocessed = self.read_json(&addr)?;
        // let metadatas: Vec<PMetadata> =
        //     serde_json::from_value(unprocessed).map_err(|e| Error::from_path(e, &addr.get_part_str()))?;
        // for metadata in metadatas {
        //     if metadata.field_name == "client_build" {
        //         return Ok(metadata.field_value.to_string());
        //     }
        // }
        Err(Error::PhbFileNoClientBuild.into())
    }
}
