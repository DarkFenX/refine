use crate::{
    ad,
    def::{ItemKey, ItemTypeId, OF},
    sol::{SolarSystem, api::DroneMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_a_item_id(&mut self, item_key: ItemKey, a_item_id: ad::AItemId) {
        let uad_item = self.uad.items.get(item_key);
        if uad_item.get_a_item_id() == a_item_id {
            return;
        }
        SolarSystem::util_remove_drone_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        uad_drone.set_a_item_id(&self.uad.src, a_item_id);
        // Update drone radius for outgoing projections
        let drone_radius = uad_drone.get_a_extras().and_then(|v| v.radius).unwrap_or(OF(0.0));
        for uad_prange in uad_drone.get_projs_mut().iter_ranges_mut() {
            uad_prange.update_src_rad(drone_radius);
        }
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_drone_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
    }
}

impl<'a> DroneMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data. If
    /// item is mutated, base item type ID is updated.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_drone_a_item_id(self.key, type_id)
    }
}
