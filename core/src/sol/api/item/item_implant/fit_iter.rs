use crate::sol::{
    FitKey, SolarSystem,
    api::{Fit, FitMut, Implant, ImplantMut, MutIter},
};

impl<'a> Fit<'a> {
    pub fn iter_implants(&self) -> impl ExactSizeIterator<Item = Implant> {
        iter_implants(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_implants(&self) -> impl ExactSizeIterator<Item = Implant> {
        iter_implants(self.sol, self.key)
    }
    pub fn iter_implants_mut(&mut self) -> MutIter<'_, ImplantMut<'_>> {
        let implant_keys = self.sol.uad.fits.get(self.key).implants.iter().copied().collect();
        MutIter::new(self.sol, implant_keys)
    }
}

fn iter_implants(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = Implant> {
    sol.uad
        .fits
        .get(fit_key)
        .implants
        .iter()
        .map(|item_key| Implant::new(sol, *item_key))
}
