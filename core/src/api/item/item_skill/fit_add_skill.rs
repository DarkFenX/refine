use std::collections::hash_map::Entry;

use crate::{
    ad::AItemId,
    api::{FitMut, ItemTypeId, SkillMut},
    err::basic::SkillEveTypeError,
    num::SkillLevel,
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitId, UFitSkill, UItem, UItemId, USkill},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_skill(
        &mut self,
        fit_uid: UFitId,
        type_aid: AItemId,
        level: SkillLevel,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<UItemId, SkillEveTypeError> {
        let fit = self.u_data.fits.get_mut(fit_uid);
        match fit.skills.entry(type_aid) {
            Entry::Vacant(entry) => {
                let item_id = self.u_data.items.alloc_id();
                let skill = USkill::new(item_id, type_aid, fit_uid, level.into(), true, &self.u_data.src);
                let item = UItem::Skill(skill);
                let skill_uid = self.u_data.items.add(item);
                entry.insert(UFitSkill {
                    skill_uid: skill_uid,
                    level,
                });
                SolarSystem::util_add_skill(&mut self.u_data, &mut self.svc, skill_uid, reuse_eupdates);
                Ok(skill_uid)
            }
            Entry::Occupied(entry) => Err(SkillEveTypeError {
                type_id: ItemTypeId::from_aid(type_aid),
                fit_id: fit.id,
                item_id: self.u_data.items.xid_by_iid(entry.get().skill_uid),
            }),
        }
    }
}

impl<'a> FitMut<'a> {
    pub fn add_skill(&mut self, type_id: ItemTypeId, level: SkillLevel) -> Result<SkillMut<'_>, AddSkillError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let skill_uid = self
            .sol
            .internal_add_skill(self.uid, type_id.into_aid(), level, &mut reuse_eupdates)?;
        Ok(SkillMut::new(self.sol, skill_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddSkillError {
    #[error("{0}")]
    SkillIdCollision(#[from] SkillEveTypeError),
}
