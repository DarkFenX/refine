use std::fmt;

use reqwest::{IntoUrl, Url, blocking::Client};

use super::error::FromSuffix;
use crate::{
    phb::{
        data::{
            PAttr, PBuff, PEffect, PFighterAbil, PItem, PItemAttrs, PItemEffects, PItemFighterAbils, PItemGroup,
            PItemList, PItemSkillMap, PItemSpaceComp, PMutaAttrMods, PMutaItemConvs,
        },
        fsd,
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
    /// a data dump, e.g. `/phobos_en-us/` and not `/phobos_en-us/fsd_binary/`.
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
    fn fetch_data(&self, suffix: &str) -> Result<serde_json::Value, Error> {
        let full_url = self.base_url.join(suffix).map_err(|e| Error::from_suffix(e, suffix))?;
        let data = self
            .client
            .get(full_url)
            .send()
            .map_err(|e| Error::from_suffix(e, suffix))?
            .error_for_status()
            .map_err(|e| Error::from_suffix(e, suffix))?
            .json()
            .map_err(|e| Error::from_suffix(e, suffix))?;
        Ok(data)
    }
    // Entity-specific processing methods
    fn process_binary_types(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_binary/types.json";
        let data = self.fetch_data(suffix)?;
        e_data.items = fsd::handle::<PItem, rc::ed::EItem>(data, suffix)?;
        Ok(())
    }
    fn process_binary_groups(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_binary/groups.json";
        let data = self.fetch_data(suffix)?;
        e_data.groups = fsd::handle::<PItemGroup, rc::ed::EItemGroup>(data, suffix)?;
        Ok(())
    }
    fn process_binary_typelist(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_binary/typelist.json";
        let data = self.fetch_data(suffix)?;
        e_data.item_lists = fsd::handle::<PItemList, rc::ed::EItemList>(data, suffix)?;
        Ok(())
    }
    fn process_binary_dogmaattributes(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_binary/dogmaattributes.json";
        let data = self.fetch_data(suffix)?;
        e_data.attrs = fsd::handle::<PAttr, rc::ed::EAttr>(data, suffix)?;
        Ok(())
    }
    fn process_binary_typedogma(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_binary/typedogma.json";
        let data = self.fetch_data(suffix)?;
        e_data.item_attrs = fsd::handle::<PItemAttrs, rc::ed::EItemAttr>(data.clone(), suffix)?;
        e_data.item_effects = fsd::handle::<PItemEffects, rc::ed::EItemEffect>(data, suffix)?;
        Ok(())
    }
    fn process_binary_dogmaeffects(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_binary/dogmaeffects.json";
        let data = self.fetch_data(suffix)?;
        e_data.effects = fsd::handle::<PEffect, rc::ed::EEffect>(data, suffix)?;
        Ok(())
    }
    fn process_lite_fighterabilities(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_lite/fighterabilities.json";
        let data = self.fetch_data(suffix)?;
        e_data.abils = fsd::handle::<PFighterAbil, rc::ed::EFighterAbil>(data, suffix)?;
        Ok(())
    }
    fn process_lite_fighterabilitiesbytype(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_lite/fighterabilitiesbytype.json";
        let data = self.fetch_data(suffix)?;
        e_data.item_abils = fsd::handle::<PItemFighterAbils, rc::ed::EItemFighterAbil>(data, suffix)?;
        Ok(())
    }
    fn process_lite_dbuffcollections(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_lite/dbuffcollections.json";
        let data = self.fetch_data(suffix)?;
        e_data.buffs = fsd::handle::<PBuff, rc::ed::EBuff>(data, suffix)?;
        Ok(())
    }
    fn process_binary_spacecomponentsbytype(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_binary/spacecomponentsbytype.json";
        let data = self.fetch_data(suffix)?;
        e_data.space_comps = fsd::handle::<PItemSpaceComp, rc::ed::EItemSpaceComp>(data, suffix)?;
        Ok(())
    }
    fn process_binary_requiredskillsfortypes(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_binary/requiredskillsfortypes.json";
        let data = self.fetch_data(suffix)?;
        e_data.item_srqs = fsd::handle::<PItemSkillMap, rc::ed::EItemSkillReq>(data, suffix)?;
        Ok(())
    }
    fn process_binary_dynamicitemattributes(&self, e_data: &mut rc::ed::EData) -> rc::ed::EResult<()> {
        let suffix = "fsd_binary/dynamicitemattributes.json";
        let data = self.fetch_data(suffix)?;
        e_data.muta_items = fsd::handle::<PMutaItemConvs, rc::ed::EMutaItemConv>(data.clone(), suffix)?;
        e_data.muta_attrs = fsd::handle::<PMutaAttrMods, rc::ed::EMutaAttrMod>(data, suffix)?;
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
        Ok(self.data_version.clone())
    }
}
