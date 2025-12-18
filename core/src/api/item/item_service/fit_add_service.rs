use crate::{
    ad::AItemId,
    api::{FitMut, ServiceMut},
    def::ItemTypeId,
    misc::ServiceState,
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitKey, UItem, UItemKey, UService},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_service(
        &mut self,
        fit_key: UFitKey,
        type_id: AItemId,
        state: ServiceState,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let item_id = self.u_data.items.alloc_id();
        let u_service = UService::new(item_id, type_id, fit_key, state, &self.u_data.src);
        let u_item = UItem::Service(u_service);
        let service_key = self.u_data.items.add(u_item);
        u_fit.services.insert(service_key);
        SolarSystem::util_add_service(&mut self.u_data, &mut self.svc, service_key, reuse_eupdates);
        service_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_service(&mut self, type_id: ItemTypeId, state: ServiceState) -> ServiceMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let service_key = self
            .sol
            .internal_add_service(self.key, type_id, state, &mut reuse_eupdates);
        ServiceMut::new(self.sol, service_key)
    }
}
