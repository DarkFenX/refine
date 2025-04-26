use std::collections::hash_map::Entry;

use crate::{
    err::basic::{SkillEveTypeError, SkillLevelError},
    sol::{
        FitKey, ItemKey, ItemTypeId, SkillLevel, SolarSystem,
        api::{FitMut, SkillMut},
        uad::{
            fit::FitSkill,
            item::{UadItem, UadSkill},
        },
    },
};

use super::misc::check_skill_level;

impl SolarSystem {
    pub(in crate::sol) fn internal_add_skill(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        level: SkillLevel,
    ) -> Result<ItemKey, SkillEveTypeError> {
        let fit = self.uad.fits.get_mut(fit_key);
        match fit.skills.entry(type_id) {
            Entry::Vacant(entry) => {
                let item_id = self.uad.items.alloc_id();
                let skill = UadSkill::new(&self.uad.src, item_id, type_id, fit_key, level, true);
                let item = UadItem::Skill(skill);
                let item_key = self.uad.items.add(item);
                entry.insert(FitSkill { item_key, level });
                self.internal_add_item_key_to_svc(item_key);
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

impl<'a> FitMut<'a> {
    pub fn add_skill(&mut self, type_id: ItemTypeId, level: SkillLevel) -> Result<SkillMut, AddSkillError> {
        check_skill_level(level)?;
        let item_key = self.sol.internal_add_skill(self.key, type_id, level)?;
        Ok(SkillMut::new(self.sol, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddSkillError {
    #[error("{0}")]
    InvalidSkillLevel(#[from] SkillLevelError),
    #[error("{0}")]
    SkillIdCollision(#[from] SkillEveTypeError),
}
