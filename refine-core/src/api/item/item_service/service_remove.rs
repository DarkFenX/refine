use crate::{
    api::ServiceMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_service(
        &mut self,
        service_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_service(&mut self.u_data, &mut self.svc, service_uid, reuse_eupdates);
        let u_service = self.u_data.items.get(service_uid).dc_service().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_service.get_fit_uid());
        u_fit.services.remove(&service_uid);
        self.u_data.items.remove(service_uid);
    }
}

impl<'a> ServiceMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_service(self.uid, &mut reuse_eupdates);
    }
}
