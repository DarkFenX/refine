use crate::{
    api::{Fighter, FighterMut, Fit, FitMut, MutIter},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn iter_fighters(&self) -> impl ExactSizeIterator<Item = Fighter<'_>> {
        iter_fighters(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_fighters(&self) -> impl ExactSizeIterator<Item = Fighter<'_>> {
        iter_fighters(self.sol, self.uid)
    }
    pub fn iter_fighters_mut(&mut self) -> MutIter<'_, FighterMut<'_>> {
        let fighter_uids = self.sol.u_data.fits.get(self.uid).fighters.iter().copied().collect();
        MutIter::new(self.sol, fighter_uids)
    }
}

fn iter_fighters(sol: &SolarSystem, fit_uid: UFitId) -> impl ExactSizeIterator<Item = Fighter<'_>> {
    let fighter_uids = sol.u_data.fits.get(fit_uid).fighters.iter();
    fighter_uids.map(|fighter_uid| Fighter::new(sol, *fighter_uid))
}
