use crate::{
    api::{Fit, FitMut, Stance, StanceMut},
    sol::SolarSystem,
    ud::UFitKey,
};

impl<'a> Fit<'a> {
    pub fn get_stance(&self) -> Option<Stance<'_>> {
        get_stance(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn get_stance(&self) -> Option<Stance<'_>> {
        get_stance(self.sol, self.key)
    }
    pub fn get_stance_mut(&mut self) -> Option<StanceMut<'_>> {
        self.sol
            .u_data
            .fits
            .get(self.key)
            .stance
            .map(|stance_key| StanceMut::new(self.sol, stance_key))
    }
}

fn get_stance(sol: &SolarSystem, fit_key: UFitKey) -> Option<Stance<'_>> {
    sol.u_data
        .fits
        .get(fit_key)
        .stance
        .map(|stance_key| Stance::new(sol, stance_key))
}
