use crate::{
    misc::ServiceState,
    sol::{
        SolarSystem,
        api::{Fit, FitMut, ItemCommon, ItemMutCommon, ItemMutSealed, ItemSealed},
    },
    ud::{UItemKey, UService},
};

pub struct Service<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> Service<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_state(&self) -> ServiceState {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for Service<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemCommon for Service<'a> {}

pub struct ServiceMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: UItemKey,
}
impl<'a> ServiceMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: UItemKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit(&self) -> Fit<'_> {
        get_fit(self.sol, self.key)
    }
    pub fn get_fit_mut(&mut self) -> FitMut<'_> {
        let fit_key = get_u_service(self.sol, self.key).get_fit_key();
        FitMut::new(self.sol, fit_key)
    }
    pub fn get_state(&self) -> ServiceState {
        get_state(self.sol, self.key)
    }
}
impl<'a> ItemSealed for ServiceMut<'a> {
    fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    fn get_key(&self) -> UItemKey {
        self.key
    }
}
impl<'a> ItemMutSealed for ServiceMut<'a> {
    fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl<'a> ItemCommon for ServiceMut<'a> {}
impl<'a> ItemMutCommon for ServiceMut<'a> {}

fn get_fit(sol: &SolarSystem, service_key: UItemKey) -> Fit<'_> {
    let fit_key = get_u_service(sol, service_key).get_fit_key();
    Fit::new(sol, fit_key)
}
fn get_state(sol: &SolarSystem, service_key: UItemKey) -> ServiceState {
    get_u_service(sol, service_key).get_service_state()
}
fn get_u_service(sol: &SolarSystem, service_key: UItemKey) -> &UService {
    sol.u_data.items.get(service_key).get_service().unwrap()
}
