use crate::{
    api::{Fit, FitMut, Stance, StanceMut},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn get_stance(&self) -> Option<Stance<'_>> {
        get_stance(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_stance(&self) -> Option<Stance<'_>> {
        get_stance(self.sol, self.uid)
    }
    pub fn get_stance_mut(&mut self) -> Option<StanceMut<'_>> {
        self.sol
            .u_data
            .fits
            .get(self.uid)
            .stance
            .map(|stance_uid| StanceMut::new(self.sol, stance_uid))
    }
}

fn get_stance(sol: &SolarSystem, fit_uid: UFitId) -> Option<Stance<'_>> {
    sol.u_data
        .fits
        .get(fit_uid)
        .stance
        .map(|stance_uid| Stance::new(sol, stance_uid))
}
