use std::fmt;

use reqwest::{blocking::get, IntoUrl, Url};

use crate::{dh, Error, Result};

use super::{
    super::{
        data::{
            Attr, Buff, Effect, FighterAbil, Item, ItemAttrs, ItemEffects, ItemFighterAbils, ItemGroup, ItemSkillMap,
            MutaAttrMods, MutaItemConvs,
        },
        fsd,
    },
    error::FromSuffix,
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
    pub fn new<U: Into<String> + Copy + IntoUrl, D: Into<String>>(
        base_url: U,
        data_version: D,
    ) -> Result<PhbHttpDHandler> {
        let base_url = base_url
            .into_url()
            .map_err(|e| Error::new(format!("failed to interpret base URL: {}", e)))?;
        match base_url.cannot_be_a_base() {
            true => Err(Error::new("passed URL cannot be used as base")),
            false => Ok(PhbHttpDHandler {
                base_url,
                data_version: data_version.into(),
            }),
        }
    }
    fn fetch_data(&self, suffix: &str) -> Result<serde_json::Value> {
        let full_url = self.base_url.join(suffix).map_err(|e| Error::from_suffix(e, suffix))?;
        let data = get(full_url)
            .map_err(|e| Error::from_suffix(e, suffix))?
            .error_for_status()
            .map_err(|e| Error::from_suffix(e, suffix))?
            .json()
            .map_err(|e| Error::from_suffix(e, suffix))?;
        Ok(data)
    }
    fn process_fsd<T, U>(&self, folder: &'static str, file: &'static str) -> dh::Result<dh::Container<U>>
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
impl dh::DataHandler for PhbHttpDHandler {
    /// Get item types.
    fn get_items(&self) -> dh::Result<dh::Container<dh::Item>> {
        self.process_fsd::<Item, dh::Item>("fsd_binary", "types")
    }
    /// Get item groups.
    fn get_item_groups(&self) -> dh::Result<dh::Container<dh::ItemGroup>> {
        self.process_fsd::<ItemGroup, dh::ItemGroup>("fsd_binary", "groups")
    }
    /// Get dogma attributes.
    fn get_attrs(&self) -> dh::Result<dh::Container<dh::Attr>> {
        self.process_fsd::<Attr, dh::Attr>("fsd_binary", "dogmaattributes")
    }
    /// Get an m:n mapping between item types and dogma attributes.
    fn get_item_attrs(&self) -> dh::Result<dh::Container<dh::ItemAttr>> {
        self.process_fsd::<ItemAttrs, dh::ItemAttr>("fsd_binary", "typedogma")
    }
    /// Get dogma effects.
    fn get_effects(&self) -> dh::Result<dh::Container<dh::Effect>> {
        self.process_fsd::<Effect, dh::Effect>("fsd_binary", "dogmaeffects")
    }
    /// Get an m:n mapping between item types and dogma effects.
    fn get_item_effects(&self) -> dh::Result<dh::Container<dh::ItemEffect>> {
        self.process_fsd::<ItemEffects, dh::ItemEffect>("fsd_binary", "typedogma")
    }
    /// Get fighter abilities.
    fn get_fighter_abils(&self) -> dh::Result<dh::Container<dh::FighterAbil>> {
        self.process_fsd::<FighterAbil, dh::FighterAbil>("fsd_lite", "fighterabilities")
    }
    /// Get an m:n mapping between item types and fighter abilities.
    fn get_item_fighter_abils(&self) -> dh::Result<dh::Container<dh::ItemFighterAbil>> {
        self.process_fsd::<ItemFighterAbils, dh::ItemFighterAbil>("fsd_lite", "fighterabilitiesbytype")
    }
    /// Get dogma buffs.
    fn get_buffs(&self) -> dh::Result<dh::Container<dh::Buff>> {
        self.process_fsd::<Buff, dh::Buff>("fsd_lite", "dbuffcollections")
    }
    /// Get item skill requirements.
    fn get_item_skill_reqs(&self) -> dh::Result<dh::Container<dh::ItemSkillReq>> {
        self.process_fsd::<ItemSkillMap, dh::ItemSkillReq>("fsd_binary", "requiredskillsfortypes")
    }
    /// Get mutaplasmid item conversions.
    fn get_muta_item_convs(&self) -> dh::Result<dh::Container<dh::MutaItemConv>> {
        self.process_fsd::<MutaItemConvs, dh::MutaItemConv>("fsd_binary", "dynamicitemattributes")
    }
    /// Get mutaplasmid item modifications.
    fn get_muta_attr_mods(&self) -> dh::Result<dh::Container<dh::MutaAttrMod>> {
        self.process_fsd::<MutaAttrMods, dh::MutaAttrMod>("fsd_binary", "dynamicitemattributes")
    }
    /// Get version of the data.
    fn get_version(&self) -> dh::Result<String> {
        Ok(self.data_version.clone())
    }
}
