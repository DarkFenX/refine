use crate::{
    api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed, ServiceState},
    sol::SolarSystem,
    ud::{UItemId, UService},
};

pub struct Service<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> Service<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_state(&self) -> ServiceState {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for Service<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemCommon for Service<'a> {}

pub struct ServiceMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UItemId,
}
impl<'a> ServiceMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.uid)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_uid = get_u_service(self.sol, self.uid).get_fit_uid();
        FitMut::new(self.sol, fit_uid)
    }
    pub fn get_state(&self) -> ServiceState {
        get_state(self.sol, self.uid)
    }
}
impl<'a> ItemSealed for ServiceMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_uid(&self) -> UItemId {
        self.uid
    }
}
impl<'a> ItemMutSealed for ServiceMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ServiceMut<'a> {}
impl<'a> ItemMutCommon for ServiceMut<'a> {}

fn get_fit(sol: &SolarSystem, service_uid: UItemId) -> Fit<'_> {
    let fit_uid = get_u_service(sol, service_uid).get_fit_uid();
    Fit::new(sol, fit_uid)
}
fn get_state(sol: &SolarSystem, service_uid: UItemId) -> ServiceState {
    get_u_service(sol, service_uid).get_service_state()
}
fn get_u_service(sol: &SolarSystem, service_uid: UItemId) -> &UService {
    sol.u_data.items.get(service_uid).dc_service().unwrap()
}
