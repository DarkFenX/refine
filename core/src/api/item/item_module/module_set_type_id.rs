use crate::{
    ad::AItemId,
    api::{ItemTypeId, ModuleMut},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_module_type_aid(
        &mut self,
        module_uid: UItemId,
        type_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(module_uid);
        if u_item.get_type_aid() == type_aid {
            return;
        }
        SolarSystem::util_remove_module_with_charge_act(&mut self.u_data, &mut self.svc, module_uid, reuse_eupdates);
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.set_type_aid(type_aid, &self.u_data.src);
        SolarSystem::util_add_module_with_charge_act(&mut self.u_data, &mut self.svc, module_uid, reuse_eupdates);
    }
}

impl<'a> ModuleMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data. If
    /// item is mutated, base item type ID is updated.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_module_type_aid(self.uid, type_id.into_aid(), &mut reuse_eupdates)
    }
}
