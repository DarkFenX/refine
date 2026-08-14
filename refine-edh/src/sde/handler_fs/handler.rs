use std::{fs::File, io::BufReader, path::PathBuf};

use super::error::SdeFsEdhError;
use crate::sde::{
    data::{
        ExtractOne, ExtractTwo, SAbil, SAttr, SBuff, SEffect, SItem, SItemAbils, SItemBuffPe, SItemBuffPt, SItemBuffSe,
        SItemBuffSl, SItemBuffSw, SItemDogma, SItemGroup, SItemList, SMetadata, SMuta, merge_item_buffs,
    },
    parsing::{extract_from_lines_one, extract_from_lines_two, first_in_lines},
};

/// Data handler which uses locally stored CCP-produced SDE in JSON Lines format
pub struct SdeFsEdh {
    base_path: PathBuf,
}
impl SdeFsEdh {
    /// Constructs filesystem EVE data handler using provided path.
    ///
    /// Path should point to the directory which contains data files.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { base_path: path.into() }
    }
}
impl std::fmt::Debug for SdeFsEdh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SdeFsEdh(\"{}\")", self.base_path.display())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Handler trait implementation
////////////////////////////////////////////////////////////////////////////////////////////////////
impl rc::ed::EveDataHandlerCore for SdeFsEdh {
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
        let reader = self.get_reader("_sde.jsonl")?;
        let metadata = first_in_lines::<SMetadata>(reader, |metadata| metadata.id == "sde")
            .map_err(|e| SdeFsEdhError::from_read_parse(e, "_sde.jsonl"))?;
        match metadata {
            Some(metadata) => Ok(metadata.build_number.to_string()),
            None => Err(SdeFsEdhError::NoBuildNumber.into()),
        }
    }
}

impl SdeFsEdh {
    fn process_types(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeFsEdhError> {
        e_data.items = self.process_one::<SItem, _>("types.jsonl")?;
        Ok(())
    }
    fn process_groups(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeFsEdhError> {
        e_data.groups = self.process_one::<SItemGroup, _>("groups.jsonl")?;
        Ok(())
    }
    fn process_type_lists(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeFsEdhError> {
        e_data.item_lists = self.process_one::<SItemList, _>("typeLists.jsonl")?;
        Ok(())
    }
    fn process_dogma_attributes(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeFsEdhError> {
        e_data.attrs = self.process_one::<SAttr, _>("dogmaAttributes.jsonl")?;
        Ok(())
    }
    fn process_dogma_effects(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeFsEdhError> {
        e_data.effects = self.process_one::<SEffect, _>("dogmaEffects.jsonl")?;
        Ok(())
    }
    fn process_type_dogma(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeFsEdhError> {
        (e_data.item_attrs, e_data.item_effects) = self.process_two::<SItemDogma, _, _>("typeDogma.jsonl")?;
        Ok(())
    }
    fn process_fighter_abilities(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeFsEdhError> {
        e_data.abils = self.process_one::<SAbil, _>("fighterAbilities.jsonl")?;
        Ok(())
    }
    fn process_fighter_abilities_by_type(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeFsEdhError> {
        e_data.item_abils = self.process_one::<SItemAbils, _>("fighterAbilitiesByType.jsonl")?;
        Ok(())
    }
    fn process_dbuff_collections(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeFsEdhError> {
        e_data.buffs = self.process_one::<SBuff, _>("dbuffCollections.jsonl")?;
        Ok(())
    }
    fn process_item_buffs(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeFsEdhError> {
        e_data.item_buffs = merge_item_buffs([
            self.process_one::<SItemBuffSw, _>("systemWideEffects.jsonl")?,
            self.process_one::<SItemBuffSe, _>("systemDbuffEmitters.jsonl")?,
            self.process_one::<SItemBuffPe, _>("appliedProximityEffects.jsonl")?,
            self.process_one::<SItemBuffPt, _>("proximityTrap.jsonl")?,
            self.process_one::<SItemBuffSl, _>("linkWithShip.jsonl")?,
        ]);
        Ok(())
    }
    fn process_dynamic_item_attributes(&self, e_data: &mut rc::ed::EData) -> Result<(), SdeFsEdhError> {
        (e_data.muta_items, e_data.muta_attrs) = self.process_two::<SMuta, _, _>("dynamicItemAttributes.jsonl")?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SdeFsEdh {
    fn process_one<SDE, EVE>(&self, file: &str) -> Result<rc::ed::EDataCont<EVE>, SdeFsEdhError>
    where
        SDE: serde::de::DeserializeOwned + ExtractOne<EVE>,
    {
        let reader = self.get_reader(file)?;
        extract_from_lines_one::<SDE, EVE>(reader).map_err(|e| SdeFsEdhError::from_read_parse(e, file))
    }
    fn process_two<SDE, EVE1, EVE2>(
        &self,
        file: &str,
    ) -> Result<(rc::ed::EDataCont<EVE1>, rc::ed::EDataCont<EVE2>), SdeFsEdhError>
    where
        SDE: serde::de::DeserializeOwned + ExtractTwo<EVE1, EVE2>,
    {
        let reader = self.get_reader(file)?;
        extract_from_lines_two::<SDE, EVE1, EVE2>(reader).map_err(|e| SdeFsEdhError::from_read_parse(e, file))
    }
    fn get_reader(&self, file: &str) -> Result<impl std::io::BufRead, SdeFsEdhError> {
        let full_path = self.base_path.join(file);
        let file_handle = File::open(full_path).map_err(|e| SdeFsEdhError::from_io(e, file))?;
        Ok(BufReader::with_capacity(64 * 1024, file_handle))
    }
}
