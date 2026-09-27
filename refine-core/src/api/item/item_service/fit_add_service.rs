use crate::{
    ad::AItemId,
    api::{FitMut, ItemTypeId, ServiceMut, ServiceState},
    sol::SolarSystem,
    ud::{UFitId, UItem, UItemId, UService},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_service(
        &mut self,
        fit_uid: UFitId,
        type_aid: AItemId,
        state: ServiceState,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let item_id = self.u_data.items.alloc_id();
        let u_service = UService::new(item_id, type_aid, fit_uid, state, &self.u_data.r_data);
        let u_item = UItem::Service(u_service);
        let service_uid = self.u_data.items.add(u_item);
        u_fit.services.insert(service_uid);
        SolarSystem::util_add_service(&mut self.u_data, &mut self.svc, service_uid, &mut self.cache.eupdates);
        service_uid
    }
}

impl<'s> FitMut<'s> {
    pub fn add_service(&mut self, type_id: ItemTypeId, state: ServiceState) -> ServiceMut<'_> {
        let service_uid = self.sol.internal_add_service(self.uid, type_id.into_aid(), state);
        ServiceMut::new(self.sol, service_uid)
    }
}
