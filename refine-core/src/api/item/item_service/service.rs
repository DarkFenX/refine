use crate::{
    Fit, ItemCommon, ServiceState, SolarSystem,
    api::ItemSealed,
    ud::{UItemId, UService},
};

pub struct Service<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UItemId,
}
impl<'s> Service<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&mut self) -> Fit<'_> {
        let fit_uid = get_u_service(self.sol, self.uid).get_fit_uid();
        Fit::new(self.sol, fit_uid)
    }
    pub fn get_state(&self) -> ServiceState {
        get_u_service(self.sol, self.uid).get_service_state()
    }
}
impl<'s> ItemSealed for Service<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol_ref(&self) -> &SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for Service<'s> {
    fn get_sol(&mut self) -> &mut SolarSystem {
        self.sol
    }
}

fn get_u_service(sol: &SolarSystem, service_uid: UItemId) -> &UService {
    sol.u_data.items.get(service_uid).dc_service().unwrap()
}
