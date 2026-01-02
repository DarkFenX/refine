use std::collections::hash_map::Entry;

use crate::{
    ad::AItemId,
    api::{FitMut, SkillMut},
    def::ItemTypeId,
    err::basic::SkillEveTypeError,
    misc::SkillLevel,
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitId, UFitSkill, UItem, UItemId, USkill},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_skill(
        &mut self,
        fit_key: UFitId,
        type_id: AItemId,
        level: SkillLevel,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<UItemId, SkillEveTypeError> {
        let fit = self.u_data.fits.get_mut(fit_key);
        match fit.skills.entry(type_id) {
            Entry::Vacant(entry) => {
                let item_id = self.u_data.items.alloc_id();
                let skill = USkill::new(item_id, type_id, fit_key, level.into(), true, &self.u_data.src);
                let item = UItem::Skill(skill);
                let skill_key = self.u_data.items.add(item);
                entry.insert(UFitSkill { skill_key, level });
                SolarSystem::util_add_skill(&mut self.u_data, &mut self.svc, skill_key, reuse_eupdates);
                Ok(skill_key)
            }
            Entry::Occupied(entry) => Err(SkillEveTypeError {
                type_id,
                fit_id: fit.id,
                item_id: self.u_data.items.eid_by_iid(entry.get().skill_key),
            }),
        }
    }
}

impl<'a> FitMut<'a> {
    pub fn add_skill(&mut self, type_id: ItemTypeId, level: SkillLevel) -> Result<SkillMut<'_>, AddSkillError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let skill_key = self
            .sol
            .internal_add_skill(self.key, type_id, level, &mut reuse_eupdates)?;
        Ok(SkillMut::new(self.sol, skill_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddSkillError {
    #[error("{0}")]
    SkillIdCollision(#[from] SkillEveTypeError),
}
