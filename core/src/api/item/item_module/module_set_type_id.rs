use crate::{
    ad::AItemId,
    api::ModuleMut,
    def::ItemTypeId,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_module_type_id(
        &mut self,
        module_key: UItemKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(module_key);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_module_with_charge_act(&mut self.u_data, &mut self.svc, module_key, reuse_eupdates);
        let u_module = self.u_data.items.get_mut(module_key).dc_module_mut().unwrap();
        u_module.set_type_id(type_id, &self.u_data.src);
        SolarSystem::util_add_module_with_charge_act(&mut self.u_data, &mut self.svc, module_key, reuse_eupdates);
    }
}

impl<'a> ModuleMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data. If
    /// item is mutated, base item type ID is updated.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_module_type_id(self.key, type_id, &mut reuse_eupdates)
    }
}
