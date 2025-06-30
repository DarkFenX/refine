use crate::{
    def::{FitId, FitKey},
    sol::SolarSystem,
};

pub struct Fit<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) key: FitKey,
}
impl<'a> Fit<'a> {
    pub(in crate::sol::api) fn new(sol: &'a SolarSystem, key: FitKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.key)
    }
}

pub struct FitMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) key: FitKey,
}
impl<'a> FitMut<'a> {
    pub(in crate::sol::api) fn new(sol: &'a mut SolarSystem, key: FitKey) -> Self {
        Self { sol, key }
    }
    pub fn get_fit_id(&self) -> FitId {
        get_fit_id(self.sol, self.key)
    }
}

fn get_fit_id(sol: &SolarSystem, fit_key: FitKey) -> FitId {
    sol.uad.fits.id_by_key(fit_key)
}
