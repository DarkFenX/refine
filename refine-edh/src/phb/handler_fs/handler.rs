use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use super::{address::Address, error::PhbFsEdhError};
use crate::phb::{
    data::{
        PAttr, PBuff, PEffect, PFighterAbil, PItem, PItemDogma, PItemFighterAbils, PItemGroup, PItemList,
        PItemSkillMap, PItemSpaceComp, PMetadata, PMuta,
    },
    parsing::{KeyMergeOne, KeyMergeTwo, extract_from_keymap_one, extract_from_keymap_two, find_in_array},
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
    fn get_data_version(&self) -> Result<String, rc::ed::err::EveDataHandlerError> {
        let addr = Address::new("phobos", "metadata");
        let reader = get_reader(&self.base_path, &addr)?;
        let metadata = find_in_array::<PMetadata>(reader, |metadata| metadata.field_name == "client_build")
            .map_err(|e| PhbFsEdhError::from_read_parse(e, addr.get_part_str()))?;
        match metadata {
            Some(metadata) => Ok(metadata.field_value.to_string()),
            None => Err(PhbFsEdhError::NoClientBuild.into()),
        }
    }
}

impl PhbFsEdh {
    fn process_built_types(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.items = process_one::<PItem, _>(&self.base_path, "fsd_built", "types")?;
        Ok(())
    }
    fn process_built_groups(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.groups = process_one::<PItemGroup, _>(&self.base_path, "fsd_built", "groups")?;
        Ok(())
    }
    fn process_built_typelist(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.item_lists = process_one::<PItemList, _>(&self.base_path, "fsd_built", "typelist")?;
        Ok(())
    }
    fn process_built_dogmaattributes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.attrs = process_one::<PAttr, _>(&self.base_path, "fsd_built", "dogmaattributes")?;
        Ok(())
    }
    fn process_built_typedogma(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        (e_data.item_attrs, e_data.item_effects) =
            process_two::<PItemDogma, _, _>(&self.base_path, "fsd_built", "typedogma")?;
        Ok(())
    }
    fn process_built_dogmaeffects(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.effects = process_one::<PEffect, _>(&self.base_path, "fsd_built", "dogmaeffects")?;
        Ok(())
    }
    fn process_lite_fighterabilities(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.abils = process_one::<PFighterAbil, _>(&self.base_path, "fsd_lite", "fighterabilities")?;
        Ok(())
    }
    fn process_lite_fighterabilitiesbytype(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.item_abils = process_one::<PItemFighterAbils, _>(&self.base_path, "fsd_lite", "fighterabilitiesbytype")?;
        Ok(())
    }
    fn process_lite_dbuffcollections(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.buffs = process_one::<PBuff, _>(&self.base_path, "fsd_lite", "dbuffcollections")?;
        Ok(())
    }
    fn process_built_spacecomponentsbytype(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.space_comps = process_one::<PItemSpaceComp, _>(&self.base_path, "fsd_built", "spacecomponentsbytype")?;
        Ok(())
    }
    fn process_built_requiredskillsfortypes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        e_data.item_srqs = process_one::<PItemSkillMap, _>(&self.base_path, "fsd_built", "requiredskillsfortypes")?;
        Ok(())
    }
    fn process_built_dynamicitemattributes(&self, e_data: &mut rc::ed::EData) -> Result<(), PhbFsEdhError> {
        (e_data.muta_items, e_data.muta_attrs) =
            process_two::<PMuta, _, _>(&self.base_path, "fsd_built", "dynamicitemattributes")?;
        Ok(())
    }
}

fn process_one<PHB, EVE>(
    base_path: &Path,
    dir: &'static str,
    file: &'static str,
) -> Result<rc::ed::EDataCont<EVE>, PhbFsEdhError>
where
    PHB: serde::de::DeserializeOwned + KeyMergeOne<EVE>,
{
    let addr = Address::new(dir, file);
    let reader = get_reader(base_path, &addr)?;
    extract_from_keymap_one::<PHB, EVE>(reader).map_err(|e| PhbFsEdhError::from_read_parse(e, addr.get_part_str()))
}
fn process_two<PHB, EVE1, EVE2>(
    base_path: &Path,
    dir: &'static str,
    file: &'static str,
) -> Result<(rc::ed::EDataCont<EVE1>, rc::ed::EDataCont<EVE2>), PhbFsEdhError>
where
    PHB: serde::de::DeserializeOwned + KeyMergeTwo<EVE1, EVE2>,
{
    let addr = Address::new(dir, file);
    let reader = get_reader(base_path, &addr)?;
    extract_from_keymap_two::<PHB, EVE1, EVE2>(reader)
        .map_err(|e| PhbFsEdhError::from_read_parse(e, addr.get_part_str()))
}
fn get_reader(base_path: &Path, addr: &Address) -> Result<impl std::io::Read, PhbFsEdhError> {
    let full_path = addr.get_full_path(base_path);
    let file = File::open(full_path).map_err(|e| PhbFsEdhError::from_io(e, addr.get_part_str()))?;
    Ok(BufReader::with_capacity(64 * 1024, file))
}
