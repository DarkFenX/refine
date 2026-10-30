use crate::{
    api::ServiceMut,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_service(&mut self, service_uid: UItemId) {
        SolarSystem::util_remove_service(&mut self.u_data, &mut self.svc, service_uid, &mut self.cache.eupdates);
        let u_service = self.u_data.items.get(service_uid).dc_service().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_service.get_fit_uid());
        u_fit.services.remove(&service_uid);
        self.internal_discard_item(service_uid, UndoStep::ServiceRemoved);
    }
}

impl<'s> ServiceMut<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_service(self.uid);
    }
}
