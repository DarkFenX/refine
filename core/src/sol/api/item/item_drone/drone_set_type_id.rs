use crate::{
    ad,
    def::ItemTypeId,
    sol::{SolarSystem, api::DroneMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_a_item_id(
        &mut self,
        item_key: UItemKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(item_key);
        if u_item.get_type_id() == a_item_id {
            return;
        }
        SolarSystem::util_remove_drone_with_projs(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        self.u_data
            .items
            .get_mut(item_key)
            .get_drone_mut()
            .unwrap()
            .set_type_id(a_item_id, reuse_eupdates, &self.u_data.src);
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, item_key);
        SolarSystem::util_add_drone_with_projs(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
    }
}

impl<'a> DroneMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data. If
    /// item is mutated, base item type ID is updated.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_drone_a_item_id(self.key, type_id, &mut reuse_eupdates)
    }
}
