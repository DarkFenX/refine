use std::fmt;

use reqwest::{IntoUrl, Url, blocking::Client};

use super::error::FromSuffix;
use crate::{
    phb::{
        data::{
            PAttr, PBuff, PEffect, PFighterAbil, PItem, PItemAttrs, PItemEffects, PItemFighterAbils, PItemGroup,
            PItemList, PItemSkillMap, PItemSpaceComp, PMutaAttrMods, PMutaItemConvs,
        },
        parsing,
    },
    util::Error,
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
    pub fn new<U>(base_url: U, data_version: String) -> Result<Self, Error>
    where
        U: IntoUrl + Copy + Into<String>,
    {
        let base_url_conv = base_url
            .into_url()
            .map_err(|e| Error::PhbHttpInvalidBaseUrl(base_url.into(), format!("failed to interpret: {e}")))?;
        match base_url_conv.cannot_be_a_base() {
            true => Err(Error::PhbHttpInvalidBaseUrl(
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
    fn get_reader(&self, suffix: &str) -> Result<impl std::io::Read, Error> {
        let full_url = self.base_url.join(suffix).map_err(|e| Error::from_suffix(e, suffix))?;
        let response = self
            .client
            .get(full_url)
            .send()
            .map_err(|e| Error::from_suffix(e, suffix))?
            .error_for_status()
            .map_err(|e| Error::from_suffix(e, suffix))?;
        Ok(response)
    }
    // Entity-specific processing methods
    fn process_built_types(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_built/types.json";
        let reader = self.get_reader(suffix)?;
        e_data.items = parsing::handle_keyed_map_one::<PItem, rc::ed::EItem>(reader, suffix)?;
        Ok(())
    }
    fn process_built_groups(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_built/groups.json";
        let reader = self.get_reader(suffix)?;
        e_data.groups = parsing::handle_keyed_map_one::<PItemGroup, rc::ed::EItemGroup>(reader, suffix)?;
        Ok(())
    }
    fn process_built_typelist(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_built/typelist.json";
        let reader = self.get_reader(suffix)?;
        e_data.item_lists = parsing::handle_keyed_map_one::<PItemList, rc::ed::EItemList>(reader, suffix)?;
        Ok(())
    }
    fn process_built_dogmaattributes(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_built/dogmaattributes.json";
        let reader = self.get_reader(suffix)?;
        e_data.attrs = parsing::handle_keyed_map_one::<PAttr, rc::ed::EAttr>(reader, suffix)?;
        Ok(())
    }
    fn process_built_typedogma(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_built/typedogma.json";
        let reader = self.get_reader(suffix)?;
        e_data.item_attrs = parsing::handle_keyed_map_one::<PItemAttrs, rc::ed::EItemAttr>(reader, suffix)?;
        let reader = self.get_reader(suffix)?;
        e_data.item_effects = parsing::handle_keyed_map_one::<PItemEffects, rc::ed::EItemEffect>(reader, suffix)?;
        Ok(())
    }
    fn process_built_dogmaeffects(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_built/dogmaeffects.json";
        let reader = self.get_reader(suffix)?;
        e_data.effects = parsing::handle_keyed_map_one::<PEffect, rc::ed::EEffect>(reader, suffix)?;
        Ok(())
    }
    fn process_lite_fighterabilities(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_lite/fighterabilities.json";
        let reader = self.get_reader(suffix)?;
        e_data.abils = parsing::handle_keyed_map_one::<PFighterAbil, rc::ed::EAbil>(reader, suffix)?;
        Ok(())
    }
    fn process_lite_fighterabilitiesbytype(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_lite/fighterabilitiesbytype.json";
        let reader = self.get_reader(suffix)?;
        e_data.item_abils = parsing::handle_keyed_map_one::<PItemFighterAbils, rc::ed::EItemAbil>(reader, suffix)?;
        Ok(())
    }
    fn process_lite_dbuffcollections(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_lite/dbuffcollections.json";
        let reader = self.get_reader(suffix)?;
        e_data.buffs = parsing::handle_keyed_map_one::<PBuff, rc::ed::EBuff>(reader, suffix)?;
        Ok(())
    }
    fn process_built_spacecomponentsbytype(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_built/spacecomponentsbytype.json";
        let reader = self.get_reader(suffix)?;
        e_data.space_comps = parsing::handle_keyed_map_one::<PItemSpaceComp, rc::ed::EItemSpaceComp>(reader, suffix)?;
        Ok(())
    }
    fn process_built_requiredskillsfortypes(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_built/requiredskillsfortypes.json";
        let reader = self.get_reader(suffix)?;
        e_data.item_srqs = parsing::handle_keyed_map_one::<PItemSkillMap, rc::ed::EItemSkillReq>(reader, suffix)?;
        Ok(())
    }
    fn process_built_dynamicitemattributes(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_built/dynamicitemattributes.json";
        let reader = self.get_reader(suffix)?;
        e_data.muta_items = parsing::handle_keyed_map_one::<PMutaItemConvs, rc::ed::EMutaItemConv>(reader, suffix)?;
        let reader = self.get_reader(suffix)?;
        e_data.muta_attrs = parsing::handle_keyed_map_one::<PMutaAttrMods, rc::ed::EMutaAttrMod>(reader, suffix)?;
        Ok(())
    }
}
impl fmt::Debug for PhbHttpEdh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhbHttpEdh(\"{}\")", self.base_url)
    }
}
impl rc::ed::EveDataHandler for PhbHttpEdh {
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
        Ok(self.data_version.clone())
    }
}
