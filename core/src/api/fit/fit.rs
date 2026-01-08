use crate::{
    sol::SolarSystem,
    ud::{FitId, UFitId},
};

pub struct Fit<'a> {
    pub(in crate::api) sol: &'a SolarSystem,
    pub(in crate::api) uid: UFitId,
}
impl<'a> Fit<'a> {
    pub(in crate::api) fn new(sol: &'a SolarSystem, uid: UFitId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.uid)
    }
}

pub struct FitMut<'a> {
    pub(in crate::api) sol: &'a mut SolarSystem,
    pub(in crate::api) uid: UFitId,
}
impl<'a> FitMut<'a> {
    pub(in crate::api) fn new(sol: &'a mut SolarSystem, uid: UFitId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.uid)
    }
}

fn get_fit_id(sol: &SolarSystem, fit_uid: UFitId) -> FitId {
    sol.u_data.fits.xid_by_iid(fit_uid)
}
