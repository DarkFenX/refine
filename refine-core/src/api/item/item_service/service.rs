use crate::{
    FitId, ItemCommon, ServiceState,
    api::ItemSealed,
    sol::SolCore,
    ud::{UItemId, UService},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Service<'sol> {
    pub(super) sol: &'sol mut SolCore,
    pub(super) uid: UItemId,
}
impl<'sol> Service<'sol> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_service(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> ServiceState {
        get_u_service(self.sol, self.uid).get_service_state()
    }
}
impl<'sol> ItemCommon for Service<'sol> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'sol> Service<'sol> {
    pub(in crate::api) fn new(sol: &'sol mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'sol> ItemSealed for Service<'sol> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol(&self) -> &SolCore {
        self.sol
    }
    fn get_sol_mut(&mut self) -> &mut SolCore {
        self.sol
    }
}

fn get_u_service(sol: &SolCore, service_uid: UItemId) -> &UService {
    sol.u_data.items.get(service_uid).dc_service().unwrap()
}
