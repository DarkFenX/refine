use crate::{
    ad::AItemId,
    def::ItemTypeId,
    sol::{SolarSystem, api::SwEffectMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_sw_effect_type_id(
        &mut self,
        sw_effect_key: UItemKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(sw_effect_key);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_sw_effect(&mut self.u_data, &mut self.svc, sw_effect_key, reuse_eupdates);
        let u_sw_effect = self.u_data.items.get_mut(sw_effect_key).get_sw_effect_mut().unwrap();
        u_sw_effect.set_type_id(type_id, &self.u_data.src);
        SolarSystem::util_add_sw_effect(&mut self.u_data, &mut self.svc, sw_effect_key, reuse_eupdates);
    }
}

impl<'a> SwEffectMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_sw_effect_type_id(self.key, type_id, &mut reuse_eupdates)
    }
}
