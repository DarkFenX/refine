use crate::{
    ad::AItemId,
    def::ItemTypeId,
    sol::{SolarSystem, api::ImplantMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_implant_type_id(
        &mut self,
        implant_key: UItemKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(implant_key);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_implant(&mut self.u_data, &mut self.svc, implant_key, reuse_eupdates);
        let u_implant = self.u_data.items.get_mut(implant_key).dc_implant_mut().unwrap();
        u_implant.set_type_id(type_id, &self.u_data.src);
        SolarSystem::util_add_implant(&mut self.u_data, &mut self.svc, implant_key, reuse_eupdates);
    }
}

impl<'a> ImplantMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_implant_type_id(self.key, type_id, &mut reuse_eupdates)
    }
}
