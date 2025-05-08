use std::fmt;

use reqwest::{IntoUrl, Url, blocking::Client};

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

use super::error::FromSuffix;

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
    fn process_fsd<T, U>(&self, folder: &'static str, file: &'static str) -> rc::ed::EResult<rc::ed::EDataCont<U>>
    where
        T: serde::de::DeserializeOwned + fsd::FsdMerge<U>,
    {
        let suffix = format!("{folder}/{file}.json");
        let json = self.fetch_data(&suffix)?;
        fsd::handle::<T, U>(json, &suffix)
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
        Ok(self.data_version.clone())
    }
}
