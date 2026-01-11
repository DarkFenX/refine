use crate::{
    ad::AItemId,
    api::{ItemTypeId, ServiceMut},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_service_type_aid(
        &mut self,
        service_uid: UItemId,
        type_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(service_uid);
        if u_item.get_type_aid() == type_aid {
            return;
        }
        SolarSystem::util_remove_service(&mut self.u_data, &mut self.svc, service_uid, reuse_eupdates);
        let u_service = self.u_data.items.get_mut(service_uid).dc_service_mut().unwrap();
        u_service.set_type_aid(type_aid, &self.u_data.src);
        SolarSystem::util_add_service(&mut self.u_data, &mut self.svc, service_uid, reuse_eupdates);
    }
}

impl<'a> ServiceMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_service_type_aid(self.uid, type_id.into_aid(), &mut reuse_eupdates)
    }
}
