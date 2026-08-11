use std::io::BufReader;

use reqwest::{Url, blocking::Client};

use super::error::{PhbHttpEdhError, PhbHttpEdhInitError};
use crate::phb::{
    data::{
        KeyMergeOne, KeyMergeTwo, PAbil, PAttr, PBuff, PEffect, PItem, PItemAbils, PItemDogma, PItemGroup, PItemList,
        PItemSkillMap, PItemSpaceComp, PMuta,
    },
    parsing::{extract_from_keymap_one, extract_from_keymap_two},
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
    pub fn try_new(base_url: impl AsRef<str>, data_version: impl Into<String>) -> Result<Self, PhbHttpEdhInitError> {
        let base_url = base_url.as_ref();
        let base_url_conv = match Url::parse(base_url) {
            Ok(base_url_conv) => base_url_conv,
            Err(error) => {
                return Err(PhbHttpEdhInitError::BaseUrlParse(base_url.to_string(), error));
            }
        };
        match base_url_conv.has_host() && !base_url_conv.cannot_be_a_base() {
            true => Ok(Self {
                base_url: base_url_conv,
                data_version: data_version.into(),
                client: Client::new(),
            }),
            false => Err(PhbHttpEdhInitError::BaseUrlNotABase(base_url.to_string())),
        }
    }
}
impl std::fmt::Debug for PhbHttpEdh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PhbHttpEdh(\"{}\")", self.base_url)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Handler trait implementation
////////////////////////////////////////////////////////////////////////////////////////////////////
impl rc::ed::EveDataHandlerInterface for PhbHttpEdh {
    fn get_data(&self) -> Result<rc::ed::EData, rc::ed::err::EveDataHandlerError> {
        let mut data = rc::ed::EData::new();
        self.process_types(&mut data)?;
        self.process_groups(&mut data)?;
        self.process_typelist(&mut data)?;
        self.process_dogmaattributes(&mut data)?;
        self.process_typedogma(&mut data)?;
        self.process_dogmaeffects(&mut data)?;
        self.process_fighterabilities(&mut data)?;
        self.process_fighterabilitiesbytype(&mut data)?;
        self.process_dbuffcollections(&mut data)?;
        self.process_spacecomponentsbytype(&mut data)?;
        self.process_requiredskillsfortypes(&mut data)?;
        self.process_dynamicitemattributes(&mut data)?;
        Ok(data)
    }
    fn get_data_version(&self) -> Result<String, rc::ed::err::EveDataHandlerError> {
        Ok(self.data_version.clone())
    }
}

impl PhbHttpEdh {
    fn process_types(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        e_data.items = self.process_one::<PItem, _>("fsd_built/types.json")?;
        Ok(())
    }
    fn process_groups(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        e_data.groups = self.process_one::<PItemGroup, _>("fsd_built/groups.json")?;
        Ok(())
    }
    fn process_typelist(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        e_data.item_lists = self.process_one::<PItemList, _>("fsd_built/typelist.json")?;
        Ok(())
    }
    fn process_dogmaattributes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        e_data.attrs = self.process_one::<PAttr, _>("fsd_built/dogmaattributes.json")?;
        Ok(())
    }
    fn process_typedogma(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        (e_data.item_attrs, e_data.item_effects) = self.process_two::<PItemDogma, _, _>("fsd_built/typedogma.json")?;
        Ok(())
    }
    fn process_dogmaeffects(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        e_data.effects = self.process_one::<PEffect, _>("fsd_built/dogmaeffects.json")?;
        Ok(())
    }
    fn process_fighterabilities(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        e_data.abils = self.process_one::<PAbil, _>("fsd_lite/fighterabilities.json")?;
        Ok(())
    }
    fn process_fighterabilitiesbytype(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        e_data.item_abils = self.process_one::<PItemAbils, _>("fsd_lite/fighterabilitiesbytype.json")?;
        Ok(())
    }
    fn process_dbuffcollections(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        e_data.buffs = self.process_one::<PBuff, _>("fsd_lite/dbuffcollections.json")?;
        Ok(())
    }
    fn process_spacecomponentsbytype(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        e_data.space_comps = self.process_one::<PItemSpaceComp, _>("fsd_built/spacecomponentsbytype.json")?;
        Ok(())
    }
    fn process_requiredskillsfortypes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        e_data.item_srqs = self.process_one::<PItemSkillMap, _>("fsd_built/requiredskillsfortypes.json")?;
        Ok(())
    }
    fn process_dynamicitemattributes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbHttpEdhError> {
        (e_data.muta_items, e_data.muta_attrs) =
            self.process_two::<PMuta, _, _>("fsd_built/dynamicitemattributes.json")?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl PhbHttpEdh {
    fn process_one<PHB, EVE>(&self, suffix: &str) -> Result<rc::ed::EDataCont<EVE>, PhbHttpEdhError>
    where
        PHB: serde::de::DeserializeOwned + KeyMergeOne<EVE>,
    {
        let reader = self.get_reader(suffix)?;
        extract_from_keymap_one::<PHB, EVE>(reader).map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))
    }
    fn process_two<PHB, EVE1, EVE2>(
        &self,
        suffix: &str,
    ) -> Result<(rc::ed::EDataCont<EVE1>, rc::ed::EDataCont<EVE2>), PhbHttpEdhError>
    where
        PHB: serde::de::DeserializeOwned + KeyMergeTwo<EVE1, EVE2>,
    {
        let reader = self.get_reader(suffix)?;
        extract_from_keymap_two::<PHB, EVE1, EVE2>(reader).map_err(|e| PhbHttpEdhError::from_read_parse(e, suffix))
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
        Ok(BufReader::with_capacity(64 * 1024, response))
    }
}
