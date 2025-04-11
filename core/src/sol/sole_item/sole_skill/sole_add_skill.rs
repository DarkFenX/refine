use std::collections::hash_map::Entry;

use crate::{
    err::basic::{FitFoundError, SkillEveTypeError, SkillLevelError},
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SkillLevel, SolarSystem,
        info::SkillInfo,
        uad::{
            fit::FitSkill,
            item::{UadItem, UadSkill},
        },
    },
};

use super::check_skill_level;

impl SolarSystem {
    pub fn add_skill(
        &mut self,
        fit_id: &FitId,
        type_id: ItemTypeId,
        level: SkillLevel,
        state: bool,
    ) -> Result<SkillInfo, AddSkillError> {
        check_skill_level(level)?;
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.add_skill_internal(fit_key, type_id, level, state)?;
        Ok(self.get_skill_info_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_skill_internal(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        level: SkillLevel,
        state: bool,
    ) -> Result<ItemKey, SkillEveTypeError> {
        let fit = self.uad.fits.get_mut(fit_key);
        match fit.skills.entry(type_id) {
            Entry::Vacant(entry) => {
                let item_id = self.uad.items.alloc_id();
                let skill = UadSkill::new(&self.uad.src, item_id, type_id, fit_key, level, state);
                let item = UadItem::Skill(skill);
                let item_key = self.uad.items.add(item);
                entry.insert(FitSkill { item_key, level });
                self.add_item_key_to_svc(item_key);
                Ok(item_key)
            }
            Entry::Occupied(entry) => Err(SkillEveTypeError {
                type_id,
                fit_id: fit.id,
                item_id: self.uad.items.id_by_key(entry.get().item_key),
            }),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddSkillError {
    #[error("{0}")]
    InvalidSkillLevel(#[from] SkillLevelError),
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
    #[error("{0}")]
    SkillIdCollision(#[from] SkillEveTypeError),
}
