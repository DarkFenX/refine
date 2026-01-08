use crate::{
    ad::AItemId,
    api::{ItemTypeId, SkillMut},
    err::basic::SkillEveTypeError,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
    util::LibGetId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_skill_type_id(
        &mut self,
        skill_uid: UItemId,
        item_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), SkillEveTypeError> {
        let u_item = self.u_data.items.get(skill_uid);
        let old_type_id = u_item.get_type_id();
        if old_type_id == item_aid {
            return Ok(());
        }
        let fit_uid = u_item.dc_skill().unwrap().get_fit_uid();
        // Check for collisions before doing anything
        let u_fit = self.u_data.fits.get(fit_uid);
        if let Some(fit_skill) = u_fit.skills.get(&item_aid) {
            return Err(SkillEveTypeError {
                type_id: ItemTypeId::from_aid(item_aid),
                fit_id: u_fit.lib_get_id(),
                item_id: self.u_data.items.xid_by_iid(fit_skill.skill_uid),
            });
        }
        // Unload skill
        SolarSystem::util_remove_skill(&mut self.u_data, &mut self.svc, skill_uid, reuse_eupdates);
        // Update type ID and reload adapted data
        let u_skill = self.u_data.items.get_mut(skill_uid).dc_skill_mut().unwrap();
        u_skill.set_type_id(item_aid, &self.u_data.src);
        // Update fit skill map
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let fit_skill = u_fit.skills.remove(&old_type_id).unwrap();
        u_fit.skills.insert(item_aid, fit_skill);
        // Load skill
        SolarSystem::util_add_skill(&mut self.u_data, &mut self.svc, skill_uid, reuse_eupdates);
        Ok(())
    }
}

impl<'a> SkillMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) -> Result<(), SetSkillTypeIdError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_skill_type_id(self.uid, type_id.into_aid(), &mut reuse_eupdates)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetSkillTypeIdError {
    #[error("{0}")]
    SkillIdCollision(#[from] SkillEveTypeError),
}
