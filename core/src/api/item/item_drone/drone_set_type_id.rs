use crate::{
    ad::AItemId,
    api::DroneMut,
    def::ItemTypeId,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_drone_type_id(
        &mut self,
        drone_key: UItemId,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(drone_key);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, drone_key, reuse_eupdates);
        self.u_data
            .items
            .get_mut(drone_key)
            .dc_drone_mut()
            .unwrap()
            .set_type_id(type_id, &self.u_data.src);
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, drone_key);
        SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_key, reuse_eupdates);
    }
}

impl<'a> DroneMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data. If
    /// item is mutated, base item type ID is updated.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_drone_type_id(self.key, type_id, &mut reuse_eupdates)
    }
}
