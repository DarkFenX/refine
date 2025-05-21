use crate::{
    ad,
    err::basic::SkillEveTypeError,
    sol::{ItemKey, ItemTypeId, SolarSystem, api::SkillMut},
    util::GetId,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_skill_a_item_id(
        &mut self,
        item_key: ItemKey,
        a_item_id: ad::AItemId,
    ) -> Result<(), SkillEveTypeError> {
        let uad_item = self.uad.items.get(item_key);
        let old_a_item_id = uad_item.get_a_item_id();
        if old_a_item_id == a_item_id {
            return Ok(());
        }
        let fit_key = uad_item.get_skill().unwrap().get_fit_key();
        // Check for collisions before doing anything
        let uad_fit = self.uad.fits.get(fit_key);
        if let Some(fit_skill) = uad_fit.skills.get(&a_item_id) {
            return Err(SkillEveTypeError {
                type_id: a_item_id,
                fit_id: uad_fit.get_id(),
                item_id: self.uad.items.id_by_key(fit_skill.item_key),
            });
        }
        // Unload skill
        SolarSystem::unload_skill(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        // Update adapted item ID and reload adapted data
        self.uad
            .items
            .get_mut(item_key)
            .get_skill_mut()
            .unwrap()
            .set_a_item_id(&self.uad.src, a_item_id);
        // Update fit skill map
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let fit_skill = uad_fit.skills.remove(&old_a_item_id).unwrap();
        uad_fit.skills.insert(a_item_id, fit_skill);
        // Load skill
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::load_skill(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        Ok(())
    }
}

impl<'a> SkillMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) -> Result<(), SetSkillTypeIdError> {
        self.sol.internal_set_skill_a_item_id(self.key, type_id)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetSkillTypeIdError {
    #[error("{0}")]
    SkillIdCollision(#[from] SkillEveTypeError),
}
