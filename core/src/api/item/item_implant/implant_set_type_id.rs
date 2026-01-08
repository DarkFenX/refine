use crate::{
    ad::AItemId,
    api::{ImplantMut, ItemTypeId},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_implant_type_id(
        &mut self,
        implant_uid: UItemId,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(implant_uid);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_implant(&mut self.u_data, &mut self.svc, implant_uid, reuse_eupdates);
        let u_implant = self.u_data.items.get_mut(implant_uid).dc_implant_mut().unwrap();
        u_implant.set_type_id(type_id, &self.u_data.src);
        SolarSystem::util_add_implant(&mut self.u_data, &mut self.svc, implant_uid, reuse_eupdates);
    }
}

impl<'a> ImplantMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_implant_type_id(self.uid, type_id.into_aid(), &mut reuse_eupdates)
    }
}
