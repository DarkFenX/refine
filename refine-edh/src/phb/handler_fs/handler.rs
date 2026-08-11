use std::{fs::File, io::BufReader, path::PathBuf};

use super::{address::Address, error::PhbFsEdhError};
use crate::phb::{
    data::{
        KeyMergeOne, KeyMergeTwo, PAbil, PAttr, PBuff, PEffect, PItem, PItemAbils, PItemDogma, PItemGroup, PItemList,
        PItemSpaceComp, PMetadata, PMuta,
    },
    parsing::{extract_from_keymap_one, extract_from_keymap_two, find_in_array},
};

/// Data handler which uses locally stored [Phobos](https://github.com/pyfa-org/Phobos) JSON dump
pub struct PhbFsEdh {
    base_path: PathBuf,
}
impl PhbFsEdh {
    /// Constructs filesystem EVE data handler using provided path.
    ///
    /// Path should point to the top-level directory of a data dump, e.g. `/phobos_en-us` and not
    /// `/phobos_en-us/fsd_built`.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { base_path: path.into() }
    }
}
impl std::fmt::Debug for PhbFsEdh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PhbFsEdh(\"{}\")", self.base_path.to_str().unwrap_or("<error>"))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Handler trait implementation
////////////////////////////////////////////////////////////////////////////////////////////////////
impl rc::ed::EveDataHandlerInterface for PhbFsEdh {
    fn get_data(&self) -> Result<rc::ed::EData, rc::ed::err::EveDataHandlerError> {
        let mut data = rc::ed::EData::new();
        self.process_types(&mut data)?;
        self.process_groups(&mut data)?;
        self.process_typelist(&mut data)?;
        self.process_dogmaattributes(&mut data)?;
        self.process_dogmaeffects(&mut data)?;
        self.process_typedogma(&mut data)?;
        self.process_fighterabilities(&mut data)?;
        self.process_fighterabilitiesbytype(&mut data)?;
        self.process_dbuffcollections(&mut data)?;
        self.process_spacecomponentsbytype(&mut data)?;
        self.process_dynamicitemattributes(&mut data)?;
        Ok(data)
    }
    fn get_data_version(&self) -> Result<String, rc::ed::err::EveDataHandlerError> {
        let addr = Address::new("phobos", "metadata");
        let reader = self.get_reader(&addr)?;
        let metadata = find_in_array::<PMetadata>(reader, |metadata| metadata.field_name == "client_build")
            .map_err(|e| PhbFsEdhError::from_read_parse(e, addr.get_part_str()))?;
        match metadata {
            Some(metadata) => Ok(metadata.field_value.to_string()),
            None => Err(PhbFsEdhError::NoClientBuild.into()),
        }
    }
}

impl PhbFsEdh {
    fn process_types(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.items = self.process_one::<PItem, _>("fsd_built", "types")?;
        Ok(())
    }
    fn process_groups(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.groups = self.process_one::<PItemGroup, _>("fsd_built", "groups")?;
        Ok(())
    }
    fn process_typelist(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.item_lists = self.process_one::<PItemList, _>("fsd_built", "typelist")?;
        Ok(())
    }
    fn process_dogmaattributes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.attrs = self.process_one::<PAttr, _>("fsd_built", "dogmaattributes")?;
        Ok(())
    }
    fn process_dogmaeffects(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.effects = self.process_one::<PEffect, _>("fsd_built", "dogmaeffects")?;
        Ok(())
    }
    fn process_typedogma(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        (e_data.item_attrs, e_data.item_effects) = self.process_two::<PItemDogma, _, _>("fsd_built", "typedogma")?;
        Ok(())
    }
    fn process_fighterabilities(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.abils = self.process_one::<PAbil, _>("fsd_lite", "fighterabilities")?;
        Ok(())
    }
    fn process_fighterabilitiesbytype(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.item_abils = self.process_one::<PItemAbils, _>("fsd_lite", "fighterabilitiesbytype")?;
        Ok(())
    }
    fn process_dbuffcollections(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.buffs = self.process_one::<PBuff, _>("fsd_lite", "dbuffcollections")?;
        Ok(())
    }
    fn process_spacecomponentsbytype(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.space_comps = self.process_one::<PItemSpaceComp, _>("fsd_built", "spacecomponentsbytype")?;
        Ok(())
    }
    fn process_dynamicitemattributes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        (e_data.muta_items, e_data.muta_attrs) =
            self.process_two::<PMuta, _, _>("fsd_built", "dynamicitemattributes")?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Shared methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl PhbFsEdh {
    fn process_one<PHB, EVE>(
        &self,
        dir: &'static str,
        file: &'static str,
    ) -> Result<rc::ed::EDataCont<EVE>, PhbFsEdhError>
    where
        PHB: serde::de::DeserializeOwned + KeyMergeOne<EVE>,
    {
        let addr = Address::new(dir, file);
        let reader = self.get_reader(&addr)?;
        extract_from_keymap_one::<PHB, EVE>(reader).map_err(|e| PhbFsEdhError::from_read_parse(e, addr.get_part_str()))
    }
    fn process_two<PHB, EVE1, EVE2>(
        &self,
        dir: &'static str,
        file: &'static str,
    ) -> Result<(rc::ed::EDataCont<EVE1>, rc::ed::EDataCont<EVE2>), PhbFsEdhError>
    where
        PHB: serde::de::DeserializeOwned + KeyMergeTwo<EVE1, EVE2>,
    {
        let addr = Address::new(dir, file);
        let reader = self.get_reader(&addr)?;
        extract_from_keymap_two::<PHB, EVE1, EVE2>(reader)
            .map_err(|e| PhbFsEdhError::from_read_parse(e, addr.get_part_str()))
    }
    fn get_reader(&self, addr: &Address) -> Result<impl std::io::Read, PhbFsEdhError> {
        let full_path = addr.get_full_path(&self.base_path);
        let file = File::open(full_path).map_err(|e| PhbFsEdhError::from_io(e, addr.get_part_str()))?;
        Ok(BufReader::with_capacity(64 * 1024, file))
    }
}
