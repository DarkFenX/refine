use crate::{
    ad::AItemId,
    def::ItemTypeId,
    sol::{SolarSystem, api::ProjEffectMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_proj_effect_type_id(
        &mut self,
        proj_effect_key: UItemKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(proj_effect_key);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_proj_effect(&mut self.u_data, &mut self.svc, proj_effect_key, reuse_eupdates);
        let u_proj_effect = self.u_data.items.get_mut(proj_effect_key).dc_proj_effect_mut().unwrap();
        u_proj_effect.set_type_id(type_id, &self.u_data.src);
        SolarSystem::util_add_proj_effect(&mut self.u_data, &mut self.svc, proj_effect_key, reuse_eupdates);
    }
}

impl<'a> ProjEffectMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_proj_effect_type_id(self.key, type_id, &mut reuse_eupdates)
    }
}
