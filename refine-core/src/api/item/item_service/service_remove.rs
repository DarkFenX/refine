use crate::{
    Service,
    sol::{SolCore, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Service<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_service(self.uid);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_remove_service(&mut self, service_uid: UItemId) {
        SolCore::util_remove_service(&mut self.u_data, &mut self.svc, service_uid, &mut self.reuse.eupdates);
        let u_service = self.u_data.items.get(service_uid).dc_service().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_service.get_fit_uid());
        u_fit.services.remove(&service_uid);
        self.internal_discard_item(service_uid, UndoStep::ServiceRemoved);
    }
}
