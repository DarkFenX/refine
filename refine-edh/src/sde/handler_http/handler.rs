use std::io::BufReader;

use reqwest::{Url, blocking::Client};

use super::error::{SdeHttpEdhError, SdeHttpEdhInitError};
use crate::sde::{
    data::{
        ExtractOne, ExtractTwo, SAbil, SAttr, SBuff, SEffect, SItem, SItemAbils, SItemBuffPe, SItemBuffPt, SItemBuffSe,
        SItemBuffSl, SItemBuffSw, SItemDogma, SItemGroup, SItemList, SMuta, merge_item_buffs,
    },
    parsing::{extract_from_lines_one, extract_from_lines_two},
};

/// Data handler which fetches CCP-produced SDE in JSON Lines format via HTTP
pub struct SdeHttpEdh {
    base_url: Url,
    data_version: String,
    client: Client,
}
impl SdeHttpEdh {
    /// Constructs HTTP EVE data handler using provided base URL and data version.
    ///
    /// This data handler assumes that data version is known before its construction.
    pub fn try_new(base_url: impl AsRef<str>, data_version: impl Into<String>) -> Result<Self, SdeHttpEdhInitError> {
        let base_url = base_url.as_ref();
        let mut base_url_conv = match Url::parse(base_url) {
            Ok(base_url_conv) => base_url_conv,
            Err(error) => {
                return Err(SdeHttpEdhInitError::BaseUrlParse(base_url.to_string(), error));
            }
        };
        if !base_url_conv.has_host() || base_url_conv.cannot_be_a_base() {
            return Err(SdeHttpEdhInitError::BaseUrlNotABase(base_url.to_string()));
        }
        // Append trailing slash if it is not there
        if !base_url_conv.path().ends_with('/') {
            let path = format!("{}/", base_url_conv.path());
            base_url_conv.set_path(&path);
        }
        Ok(Self {
            base_url: base_url_conv,
            data_version: data_version.into(),
            client: Client::new(),
        })
    }
}
impl std::fmt::Debug for SdeHttpEdh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SdeHttpEdh(\"{}\")", self.base_url)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Handler trait implementation
////////////////////////////////////////////////////////////////////////////////////////////////////
impl rc::ed::EveDataHandlerInterface for SdeHttpEdh {
    fn get_data(&self) -> Result<rc::ed::EData, rc::ed::err::EveDataHandlerError> {
        let mut data = rc::ed::EData::new();
        self.process_types(&mut data)?;
        self.process_groups(&mut data)?;
        self.process_type_lists(&mut data)?;
        self.process_dogma_attributes(&mut data)?;
        self.process_dogma_effects(&mut data)?;
        self.process_type_dogma(&mut data)?;
        self.process_fighter_abilities(&mut data)?;
        self.process_fighter_abilities_by_type(&mut data)?;
        self.process_dbuff_collections(&mut data)?;
        self.process_item_buffs(&mut data)?;
        self.process_dynamic_item_attributes(&mut data)?;
        Ok(data)
    }
    fn get_data_version(&self) -> Result<String, rc::ed::err::EveDataHandlerError> {
        Ok(self.data_version.clone())
    }
}

impl SdeHttpEdh {
    fn process_types(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeHttpEdhError> {
        e_data.items = self.process_one::<SItem, _>("types.jsonl")?;
        Ok(())
    }
    fn process_groups(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeHttpEdhError> {
        e_data.groups = self.process_one::<SItemGroup, _>("groups.jsonl")?;
        Ok(())
    }
    fn process_type_lists(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeHttpEdhError> {
        e_data.item_lists = self.process_one::<SItemList, _>("typeLists.jsonl")?;
        Ok(())
    }
    fn process_dogma_attributes(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeHttpEdhError> {
        e_data.attrs = self.process_one::<SAttr, _>("dogmaAttributes.jsonl")?;
        Ok(())
    }
    fn process_dogma_effects(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeHttpEdhError> {
        e_data.effects = self.process_one::<SEffect, _>("dogmaEffects.jsonl")?;
        Ok(())
    }
    fn process_type_dogma(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeHttpEdhError> {
        (e_data.item_attrs, e_data.item_effects) = self.process_two::<SItemDogma, _, _>("typeDogma.jsonl")?;
        Ok(())
    }
    fn process_fighter_abilities(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeHttpEdhError> {
        e_data.abils = self.process_one::<SAbil, _>("fighterAbilities.jsonl")?;
        Ok(())
    }
    fn process_fighter_abilities_by_type(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeHttpEdhError> {
        e_data.item_abils = self.process_one::<SItemAbils, _>("fighterAbilitiesByType.jsonl")?;
        Ok(())
    }
    fn process_dbuff_collections(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeHttpEdhError> {
        e_data.buffs = self.process_one::<SBuff, _>("dbuffCollections.jsonl")?;
        Ok(())
    }
    fn process_item_buffs(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeHttpEdhError> {
        e_data.item_buffs = merge_item_buffs([
            self.process_one::<SItemBuffSw, _>("systemWideEffects.jsonl")?,
            self.process_one::<SItemBuffSe, _>("systemDbuffEmitters.jsonl")?,
            self.process_one::<SItemBuffPe, _>("appliedProximityEffects.jsonl")?,
            self.process_one::<SItemBuffPt, _>("proximityTrap.jsonl")?,
            self.process_one::<SItemBuffSl, _>("linkWithShip.jsonl")?,
        ]);
        Ok(())
    }
    fn process_dynamic_item_attributes(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeHttpEdhError> {
        (e_data.muta_items, e_data.muta_attrs) = self.process_two::<SMuta, _, _>("dynamicItemAttributes.jsonl")?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SdeHttpEdh {
    fn process_one<SDE, EVE>(&self, suffix: &str) -> Result<rc::ed::EDataCont<EVE>, SdeHttpEdhError>
    where
        SDE: serde::de::DeserializeOwned + ExtractOne<EVE>,
    {
        let reader = self.get_reader(suffix)?;
        extract_from_lines_one::<SDE, EVE>(reader).map_err(|e| SdeHttpEdhError::from_read_parse(e, suffix))
    }
    fn process_two<SDE, EVE1, EVE2>(
        &self,
        suffix: &str,
    ) -> Result<(rc::ed::EDataCont<EVE1>, rc::ed::EDataCont<EVE2>), SdeHttpEdhError>
    where
        SDE: serde::de::DeserializeOwned + ExtractTwo<EVE1, EVE2>,
    {
        let reader = self.get_reader(suffix)?;
        extract_from_lines_two::<SDE, EVE1, EVE2>(reader).map_err(|e| SdeHttpEdhError::from_read_parse(e, suffix))
    }
    fn get_reader(&self, suffix: &str) -> Result<impl std::io::BufRead, SdeHttpEdhError> {
        let full_url = self
            .base_url
            .join(suffix)
            .map_err(|e| SdeHttpEdhError::from_url(e, suffix))?;
        let response = self
            .client
            .get(full_url)
            .send()
            .map_err(|e| SdeHttpEdhError::from_reqwest(e, suffix))?
            .error_for_status()
            .map_err(|e| SdeHttpEdhError::from_reqwest(e, suffix))?;
        Ok(BufReader::with_capacity(64 * 1024, response))
    }
}
