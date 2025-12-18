use crate::{
    api::ServiceMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_service(
        &mut self,
        service_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_service(&mut self.u_data, &mut self.svc, service_key, reuse_eupdates);
        let u_service = self.u_data.items.get(service_key).dc_service().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_service.get_fit_key());
        u_fit.services.remove(&service_key);
        self.u_data.items.remove(service_key);
    }
}

impl<'a> ServiceMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_service(self.key, &mut reuse_eupdates);
    }
}
