use crate::{def::FitId, sol::SolarSystem, ud::UFitId};

pub struct Fit<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UFitId,
}
impl<'a> Fit<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, key: UFitId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.key)
    }
}

pub struct FitMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UFitId,
}
impl<'a> FitMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UFitId) -> Self {
        Self { sol, key }
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.key)
    }
}

fn get_fit_id(sol: &SolarSystem, fit_key: UFitId) -> FitId {
    sol.u_data.fits.ext_id_by_int_id(fit_key)
}
