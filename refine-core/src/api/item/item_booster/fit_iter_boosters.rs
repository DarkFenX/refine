use crate::{
    api::{Booster, BoosterMut, Fit, FitMut, MutIter},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn iter_boosters(&self) -> impl ExactSizeIterator<Item = Booster<'_>> {
        iter_boosters(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_boosters(&self) -> impl ExactSizeIterator<Item = Booster<'_>> {
        iter_boosters(self.sol, self.uid)
    }
    pub fn iter_boosters_mut(&mut self) -> MutIter<'_, BoosterMut<'_>> {
        let booster_uids = self.sol.u_data.fits.get(self.uid).boosters.iter().copied().collect();
        MutIter::new(self.sol, booster_uids)
    }
}

fn iter_boosters(sol: &SolarSystem, fit_uid: UFitId) -> impl ExactSizeIterator<Item = Booster<'_>> {
    let booster_uids = sol.u_data.fits.get(fit_uid).boosters.iter();
    booster_uids.map(|booster_uid| Booster::new(sol, *booster_uid))
}
