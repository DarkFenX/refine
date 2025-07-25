use crate::{
    ad,
    def::ItemTypeId,
    sol::{SolarSystem, api::FwEffectMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fw_effect_a_item_id(
        &mut self,
        item_key: UItemKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(item_key);
        if u_item.get_a_item_id() == a_item_id {
            return;
        }
        SolarSystem::util_remove_fw_effect(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        self.u_data
            .items
            .get_mut(item_key)
            .get_fw_effect_mut()
            .unwrap()
            .set_a_item_id(a_item_id, reuse_eupdates, &self.u_data.src);
        SolarSystem::util_add_fw_effect(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
    }
}

impl<'a> FwEffectMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_fw_effect_a_item_id(self.key, type_id, &mut reuse_eupdates)
    }
}
