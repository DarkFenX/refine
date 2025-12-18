use crate::{def::FitId, sol::SolarSystem, ud::UFitKey};

pub struct Fit<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) key: UFitKey,
}
impl<'a> Fit<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, key: UFitKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.key)
    }
}

pub struct FitMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) key: UFitKey,
}
impl<'a> FitMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, key: UFitKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.key)
    }
}

fn get_fit_id(sol: &SolarSystem, fit_key: UFitKey) -> FitId {
    sol.u_data.fits.id_by_key(fit_key)
}
