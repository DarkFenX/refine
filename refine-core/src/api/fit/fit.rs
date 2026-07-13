use crate::{
    sol::SolarSystem,
    ud::{FitId, UFitId},
};

pub struct Fit<'s> {
    pub(in crate::api) sol: &'s SolarSystem,
    pub(in crate::api) uid: UFitId,
}
impl<'s> Fit<'s> {
    pub(in crate::api) fn new(sol: &'s SolarSystem, uid: UFitId) -> Self {
        Self { sol, uid }
    }
    pub fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.uid)
    }
}

pub struct FitMut<'s> {
    pub(in crate::api) sol: &'s mut SolarSystem,
    pub(in crate::api) uid: UFitId,
}
impl<'s> FitMut<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UFitId) -> Self {
        Self { sol, uid }
    }
    pub fn get_sol(&self) -> &SolarSystem {
        self.sol
    }
    pub fn get_sol_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.uid)
    }
}

fn get_fit_id(sol: &SolarSystem, fit_uid: UFitId) -> FitId {
    sol.u_data.fits.ext_id_by_int_id(fit_uid)
}
