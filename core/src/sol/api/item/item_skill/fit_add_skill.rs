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
    uad::{UadEffectUpdates, UadFitKey, UadFitSkill, UadItem, UadItemKey, UadSkill},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_skill(
        &mut self,
        fit_key: UadFitKey,
        a_item_id: ad::AItemId,
        level: SkillLevel,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> Result<UadItemKey, SkillEveTypeError> {
        let fit = self.uad.fits.get_mut(fit_key);
        match fit.skills.entry(a_item_id) {
            Entry::Vacant(entry) => {
                let item_id = self.uad.items.alloc_id();
                let skill = UadSkill::new(
                    item_id,
                    a_item_id,
                    fit_key,
                    level.into(),
                    true,
                    &self.uad.src,
                    reuse_eupdates,
                );
                let item = UadItem::Skill(skill);
                let item_key = self.uad.items.add(item);
                entry.insert(UadFitSkill { item_key, level });
                SolarSystem::util_add_skill(&self.uad, &mut self.svc, item_key, reuse_eupdates);
                Ok(item_key)
            }
            Entry::Occupied(entry) => Err(SkillEveTypeError {
                type_id: a_item_id,
                fit_id: fit.id,
                item_id: self.uad.items.id_by_key(entry.get().item_key),
            }),
        }
    }
}

impl<'a> FitMut<'a> {
    pub fn add_skill(&mut self, type_id: ItemTypeId, level: SkillLevel) -> Result<SkillMut<'_>, AddSkillError> {
        let mut reuse_eupdates = UadEffectUpdates::new();
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
