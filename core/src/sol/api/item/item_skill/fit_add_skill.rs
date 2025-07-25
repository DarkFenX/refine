use std::collections::hash_map::Entry;

use crate::{
    ad,
    def::ItemTypeId,
    err::basic::SkillEveTypeError,
    misc::SkillLevel,
    sol::{
        SolarSystem,
        api::{FitMut, SkillMut},
    },
    ud::{UEffectUpdates, UFitKey, UFitSkill, UItem, UItemKey, USkill},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_skill(
        &mut self,
        fit_key: UFitKey,
        a_item_id: ad::AItemId,
        level: SkillLevel,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<UItemKey, SkillEveTypeError> {
        let fit = self.u_data.fits.get_mut(fit_key);
        match fit.skills.entry(a_item_id) {
            Entry::Vacant(entry) => {
                let item_id = self.u_data.items.alloc_id();
                let skill = USkill::new(
                    item_id,
                    a_item_id,
                    fit_key,
                    level.into(),
                    true,
                    &self.u_data.src,
                    reuse_eupdates,
                );
                let item = UItem::Skill(skill);
                let item_key = self.u_data.items.add(item);
                entry.insert(UFitSkill { item_key, level });
                SolarSystem::util_add_skill(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
                Ok(item_key)
            }
            Entry::Occupied(entry) => Err(SkillEveTypeError {
                type_id: a_item_id,
                fit_id: fit.id,
                item_id: self.u_data.items.id_by_key(entry.get().item_key),
            }),
        }
    }
}

impl<'a> FitMut<'a> {
    pub fn add_skill(&mut self, type_id: ItemTypeId, level: SkillLevel) -> Result<SkillMut<'_>, AddSkillError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self
            .sol
            .internal_add_skill(self.key, type_id, level, &mut reuse_eupdates)?;
        Ok(SkillMut::new(self.sol, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddSkillError {
    #[error("{0}")]
    SkillIdCollision(#[from] SkillEveTypeError),
}
