use crate::{
    ad::AItemId,
    api::{FitMut, ItemTypeId, ServiceMut, ServiceState},
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitId, UItem, UItemId, UService},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_service(
        &mut self,
        fit_uid: UFitId,
        type_aid: AItemId,
        state: ServiceState,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let item_id = self.u_data.items.alloc_id();
        let u_service = UService::new(item_id, type_aid, fit_uid, state, &self.u_data.src);
        let u_item = UItem::Service(u_service);
        let service_uid = self.u_data.items.add(u_item);
        u_fit.services.insert(service_uid);
        SolarSystem::util_add_service(&mut self.u_data, &mut self.svc, service_uid, reuse_eupdates);
        service_uid
    }
}

impl<'a> FitMut<'a> {
    pub fn add_service(&mut self, type_id: ItemTypeId, state: ServiceState) -> ServiceMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let service_uid = self
            .sol
            .internal_add_service(self.uid, type_id.into_aid(), state, &mut reuse_eupdates);
        ServiceMut::new(self.sol, service_uid)
    }
}
