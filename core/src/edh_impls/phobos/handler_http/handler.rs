use std::fmt;

use reqwest::{blocking::get, IntoUrl, Url};

use crate::{
    edh,
    edh_impls::phobos::{
        data::{
            Attr, Buff, Effect, FighterAbil, Item, ItemAttrs, ItemEffects, ItemFighterAbils, ItemGroup, ItemSkillMap,
            MutaAttrMods, MutaItemConvs,
        },
        fsd,
        handler_http::error::FromSuffix,
    },
    edt,
    util::{Error, ErrorKind, IntError, IntResult, Result},
};

/// Data handler which uses HTTP-served [Phobos](https://github.com/pyfa-org/Phobos) JSON dump
pub struct PhbHttpDHandler {
    base_url: Url,
    data_version: String,
}
impl PhbHttpDHandler {
    /// Constructs new `PhbHttpDHandler` using provided base URL and data version.
    ///
    /// URL should end with a trailing slash, and should point to the top-level directory of
    /// a data dump, e.g. `/phobos_en-us/` and not `/phobos_en-us/fsd_binary/`.
    pub fn new<U: IntoUrl + Copy + Into<String>>(base_url: U, data_version: String) -> Result<Self> {
        let base_url_conv = base_url.into_url().map_err(|e| {
            Error::new(ErrorKind::DhHttpInvalidBaseUrl(
                base_url.into(),
                format!("failed to interpret: {}", e),
            ))
        })?;
        match base_url_conv.cannot_be_a_base() {
            true => Err(Error::new(ErrorKind::DhHttpInvalidBaseUrl(
                base_url.into(),
                "cannot be used as base".to_string(),
            ))),
            false => Ok(Self {
                base_url: base_url_conv,
                data_version,
            }),
        }
    }
    fn fetch_data(&self, suffix: &str) -> IntResult<serde_json::Value> {
        let full_url = self
            .base_url
            .join(suffix)
            .map_err(|e| IntError::from_suffix(e, suffix))?;
        let data = get(full_url)
            .map_err(|e| IntError::from_suffix(e, suffix))?
            .error_for_status()
            .map_err(|e| IntError::from_suffix(e, suffix))?
            .json()
            .map_err(|e| IntError::from_suffix(e, suffix))?;
        Ok(data)
    }
    fn process_fsd<T, U>(&self, folder: &'static str, file: &'static str) -> edh::Result<edh::Container<U>>
    where
        T: serde::de::DeserializeOwned + fsd::FsdMerge<U>,
    {
        let suffix = format!("{}/{}.json", folder, file);
        let json = self.fetch_data(&suffix)?;
        fsd::handle::<T, U>(json)
    }
}
impl fmt::Debug for PhbHttpDHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhbHttpDHandler(\"{}\")", self.base_url.to_string())
    }
}
impl edh::EveDataHandler for PhbHttpDHandler {
    /// Get item types.
    fn get_items(&self) -> edh::Result<edh::Container<edt::Item>> {
        self.process_fsd::<Item, edt::Item>("fsd_binary", "types")
    }
    /// Get item groups.
    fn get_item_groups(&self) -> edh::Result<edh::Container<edt::ItemGroup>> {
        self.process_fsd::<ItemGroup, edt::ItemGroup>("fsd_binary", "groups")
    }
    /// Get dogma attributes.
    fn get_attrs(&self) -> edh::Result<edh::Container<edt::Attr>> {
        self.process_fsd::<Attr, edt::Attr>("fsd_binary", "dogmaattributes")
    }
    /// Get an m:n mapping between item types and dogma attributes.
    fn get_item_attrs(&self) -> edh::Result<edh::Container<edt::ItemAttr>> {
        self.process_fsd::<ItemAttrs, edt::ItemAttr>("fsd_binary", "typedogma")
    }
    /// Get dogma effects.
    fn get_effects(&self) -> edh::Result<edh::Container<edt::Effect>> {
        self.process_fsd::<Effect, edt::Effect>("fsd_binary", "dogmaeffects")
    }
    /// Get an m:n mapping between item types and dogma effects.
    fn get_item_effects(&self) -> edh::Result<edh::Container<edt::ItemEffect>> {
        self.process_fsd::<ItemEffects, edt::ItemEffect>("fsd_binary", "typedogma")
    }
    /// Get fighter abilities.
    fn get_fighter_abils(&self) -> edh::Result<edh::Container<edt::FighterAbil>> {
        self.process_fsd::<FighterAbil, edt::FighterAbil>("fsd_lite", "fighterabilities")
    }
    /// Get an m:n mapping between item types and fighter abilities.
    fn get_item_fighter_abils(&self) -> edh::Result<edh::Container<edt::ItemFighterAbil>> {
        self.process_fsd::<ItemFighterAbils, edt::ItemFighterAbil>("fsd_lite", "fighterabilitiesbytype")
    }
    /// Get dogma buffs.
    fn get_buffs(&self) -> edh::Result<edh::Container<edt::Buff>> {
        self.process_fsd::<Buff, edt::Buff>("fsd_lite", "dbuffcollections")
    }
    /// Get item skill requirements.
    fn get_item_skill_reqs(&self) -> edh::Result<edh::Container<edt::ItemSkillReq>> {
        self.process_fsd::<ItemSkillMap, edt::ItemSkillReq>("fsd_binary", "requiredskillsfortypes")
    }
    /// Get mutaplasmid item conversions.
    fn get_muta_item_convs(&self) -> edh::Result<edh::Container<edt::MutaItemConv>> {
        self.process_fsd::<MutaItemConvs, edt::MutaItemConv>("fsd_binary", "dynamicitemattributes")
    }
    /// Get mutaplasmid item modifications.
    fn get_muta_attr_mods(&self) -> edh::Result<edh::Container<edt::MutaAttrMod>> {
        self.process_fsd::<MutaAttrMods, edt::MutaAttrMod>("fsd_binary", "dynamicitemattributes")
    }
    /// Get version of the data.
    fn get_version(&self) -> edh::Result<String> {
        Ok(self.data_version.clone())
    }
}
