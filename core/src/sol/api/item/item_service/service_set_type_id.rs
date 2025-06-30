use crate::{
    ad,
    def::{ItemKey, ItemTypeId},
    sol::{SolarSystem, api::ServiceMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_service_a_item_id(&mut self, item_key: ItemKey, a_item_id: ad::AItemId) {
        let uad_item = self.uad.items.get(item_key);
        if uad_item.get_a_item_id() == a_item_id {
            return;
        }
        SolarSystem::util_remove_service(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        self.uad
            .items
            .get_mut(item_key)
            .get_service_mut()
            .unwrap()
            .set_a_item_id(&self.uad.src, a_item_id);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_service(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
    }
}

impl<'a> ServiceMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_service_a_item_id(self.key, type_id)
    }
}
