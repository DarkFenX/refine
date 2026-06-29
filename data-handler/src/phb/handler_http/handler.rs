use std::fmt;

use reqwest::{IntoUrl, Url, blocking::Client};

use super::error::{PhbHttpEdhError, PhbHttpEdhInitError};
use crate::phb::{
    data::{
        PAttr, PBuff, PEffect, PFighterAbil, PItem, PItemDogma, PItemFighterAbils, PItemGroup, PItemList,
        PItemSkillMap, PItemSpaceComp, PMuta,
    },
    parsing::{handle_keymap_one, handle_keymap_two},
};

/// Data handler which fetches [Phobos](https://github.com/pyfa-org/Phobos) JSON dump via HTTP
pub struct PhbHttpEdh {
    base_url: Url,
    data_version: String,
    client: Client,
}
impl PhbHttpEdh {
    /// Constructs HTTP EVE data handler using provided base URL and data version.
    ///
    /// URL should end with a trailing slash, and should point to the top-level directory of
    /// a data dump, e.g. `/phobos_en-us/` and not `/phobos_en-us/fsd_built/`.
    ///
    /// This data handler assumes that data version is known before its construction.
    pub fn try_new<U>(base_url: U, data_version: String) -> Result<Self, PhbHttpEdhInitError>
    where
        U: IntoUrl + Copy + Into<String>,
    {
        let base_url_conv = base_url.into_url().map_err(|e| {
            PhbHttpEdhInitError::PhbHttpInvalidBaseUrl(base_url.into(), format!("failed to interpret: {e}"))
        })?;
        match base_url_conv.cannot_be_a_base() {
            true => Err(PhbHttpEdhInitError::PhbHttpInvalidBaseUrl(
                base_url.into(),
                "cannot be used as base".to_string(),
            )),
            false => Ok(Self {
                base_url: base_url_conv,
                data_version,
                client: Client::new(),
            }),
        }
    }
    fn get_reader(&self, suffix: &str) -> Result<impl std::io::Read, PhbHttpEdhError> {
        let full_url = self
            .base_url
            .join(suffix)
            .map_err(|e| PhbHttpEdhError::from_url(e, suffix))?;
        let response = self
            .client
            .get(full_url)
            .send()
            .map_err(|e| PhbHttpEdhError::from_reqwest(e, suffix))?
            .error_for_status()
            .map_err(|e| PhbHttpEdhError::from_reqwest(e, suffix))?;
        Ok(response)
    }
    // Entity-specific processing methods
    fn process_built_types(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        let suffix = "fsd_built/types.json";
        let reader = self.get_reader(suffix)?;
        e_data.items = handle_keymap_one::<PItem, rc::ed::EItem>(reader)
            .map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))?;
        Ok(())
    }
    fn process_built_groups(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        let suffix = "fsd_built/groups.json";
        let reader = self.get_reader(suffix)?;
        e_data.groups = handle_keymap_one::<PItemGroup, rc::ed::EItemGroup>(reader)
            .map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))?;
        Ok(())
    }
    fn process_built_typelist(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        let suffix = "fsd_built/typelist.json";
        let reader = self.get_reader(suffix)?;
        e_data.item_lists = handle_keymap_one::<PItemList, rc::ed::EItemList>(reader)
            .map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))?;
        Ok(())
    }
    fn process_built_dogmaattributes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        let suffix = "fsd_built/dogmaattributes.json";
        let reader = self.get_reader(suffix)?;
        e_data.attrs = handle_keymap_one::<PAttr, rc::ed::EAttr>(reader)
            .map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))?;
        Ok(())
    }
    fn process_built_typedogma(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        let suffix = "fsd_built/typedogma.json";
        let reader = self.get_reader(suffix)?;
        (e_data.item_attrs, e_data.item_effects) =
            handle_keymap_two::<PItemDogma, rc::ed::EItemAttr, rc::ed::EItemEffect>(reader)
                .map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))?;
        Ok(())
    }
    fn process_built_dogmaeffects(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        let suffix = "fsd_built/dogmaeffects.json";
        let reader = self.get_reader(suffix)?;
        e_data.effects = handle_keymap_one::<PEffect, rc::ed::EEffect>(reader)
            .map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))?;
        Ok(())
    }
    fn process_lite_fighterabilities(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        let suffix = "fsd_lite/fighterabilities.json";
        let reader = self.get_reader(suffix)?;
        e_data.abils = handle_keymap_one::<PFighterAbil, rc::ed::EAbil>(reader)
            .map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))?;
        Ok(())
    }
    fn process_lite_fighterabilitiesbytype(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        let suffix = "fsd_lite/fighterabilitiesbytype.json";
        let reader = self.get_reader(suffix)?;
        e_data.item_abils = handle_keymap_one::<PItemFighterAbils, rc::ed::EItemAbil>(reader)
            .map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))?;
        Ok(())
    }
    fn process_lite_dbuffcollections(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        let suffix = "fsd_lite/dbuffcollections.json";
        let reader = self.get_reader(suffix)?;
        e_data.buffs = handle_keymap_one::<PBuff, rc::ed::EBuff>(reader)
            .map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))?;
        Ok(())
    }
    fn process_built_spacecomponentsbytype(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        let suffix = "fsd_built/spacecomponentsbytype.json";
        let reader = self.get_reader(suffix)?;
        e_data.space_comps = handle_keymap_one::<PItemSpaceComp, rc::ed::EItemSpaceComp>(reader)
            .map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))?;
        Ok(())
    }
    fn process_built_requiredskillsfortypes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        let suffix = "fsd_built/requiredskillsfortypes.json";
        let reader = self.get_reader(suffix)?;
        e_data.item_srqs = handle_keymap_one::<PItemSkillMap, rc::ed::EItemSkillReq>(reader)
            .map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))?;
        Ok(())
    }
    fn process_built_dynamicitemattributes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        let suffix = "fsd_built/dynamicitemattributes.json";
        let reader = self.get_reader(suffix)?;
        (e_data.muta_items, e_data.muta_attrs) =
            handle_keymap_two::<PMuta, rc::ed::EMutaItemConv, rc::ed::EMutaAttrMod>(reader)
                .map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))?;
        Ok(())
    }
}
impl fmt::Debug for PhbHttpEdh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhbHttpEdh(\"{}\")", self.base_url)
    }
}
impl rc::ed::EveDataHandler for PhbHttpEdh {
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
        Ok(self.data_version.clone())
    }
}
