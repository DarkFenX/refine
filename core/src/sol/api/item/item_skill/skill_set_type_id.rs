use crate::{
    ad::AItemId,
    def::ItemTypeId,
    err::basic::SkillEveTypeError,
    sol::{SolarSystem, api::SkillMut},
    ud::{UEffectUpdates, UItemKey},
    util::GetId,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_skill_type_id(
        &mut self,
        skill_key: UItemKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), SkillEveTypeError> {
        let u_item = self.u_data.items.get(skill_key);
        let old_type_id = u_item.get_type_id();
        if old_type_id == type_id {
            return Ok(());
        }
        let fit_key = u_item.dc_skill().unwrap().get_fit_key();
        // Check for collisions before doing anything
        let u_fit = self.u_data.fits.get(fit_key);
        if let Some(fit_skill) = u_fit.skills.get(&type_id) {
            return Err(SkillEveTypeError {
                type_id,
                fit_id: u_fit.get_id(),
                item_id: self.u_data.items.id_by_key(fit_skill.skill_key),
            });
        }
        // Unload skill
        SolarSystem::util_remove_skill(&mut self.u_data, &mut self.svc, skill_key, reuse_eupdates);
        // Update type ID and reload adapted data
        let u_skill = self.u_data.items.get_mut(skill_key).dc_skill_mut().unwrap();
        u_skill.set_type_id(type_id, &self.u_data.src);
        // Update fit skill map
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let fit_skill = u_fit.skills.remove(&old_type_id).unwrap();
        u_fit.skills.insert(type_id, fit_skill);
        // Load skill
        SolarSystem::util_add_skill(&mut self.u_data, &mut self.svc, skill_key, reuse_eupdates);
        Ok(())
    }
}

impl<'a> SkillMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) -> Result<(), SetSkillTypeIdError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_skill_type_id(self.key, type_id, &mut reuse_eupdates)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetSkillTypeIdError {
    #[error("{0}")]
    SkillIdCollision(#[from] SkillEveTypeError),
}
