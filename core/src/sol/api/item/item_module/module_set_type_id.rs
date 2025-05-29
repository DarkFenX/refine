use crate::{
    ad,
    sol::{ItemKey, ItemTypeId, SolarSystem, api::ModuleMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_module_a_item_id(&mut self, item_key: ItemKey, a_item_id: ad::AItemId) {
        let uad_item = self.uad.items.get(item_key);
        if uad_item.get_a_item_id() == a_item_id {
            return;
        }
        SolarSystem::util_remove_module_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        self.uad
            .items
            .get_mut(item_key)
            .get_module_mut()
            .unwrap()
            .set_a_item_id(&self.uad.src, a_item_id);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_module_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
    }
}

impl<'a> ModuleMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data. If
    /// item is mutated, base item type ID is updated.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_module_a_item_id(self.key, type_id)
    }
}
