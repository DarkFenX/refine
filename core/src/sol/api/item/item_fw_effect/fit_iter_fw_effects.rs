use crate::{
    def::FitKey,
    sol::{
        SolarSystem,
        api::{Fit, FitMut, FwEffect, FwEffectMut, MutIter},
    },
};

impl<'a> Fit<'a> {
    pub fn iter_fw_effects(&self) -> impl ExactSizeIterator<Item = FwEffect<'_>> {
        iter_fw_effects(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_fw_effects(&self) -> impl ExactSizeIterator<Item = FwEffect<'_>> {
        iter_fw_effects(self.sol, self.key)
    }
    pub fn iter_fw_effects_mut(&mut self) -> MutIter<'_, FwEffectMut<'_>> {
        let implant_keys = self.sol.uad.fits.get(self.key).fw_effects.iter().copied().collect();
        MutIter::new(self.sol, implant_keys)
    }
}

fn iter_fw_effects(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = FwEffect<'_>> {
    sol.uad
        .fits
        .get(fit_key)
        .fw_effects
        .iter()
        .map(|item_key| FwEffect::new(sol, *item_key))
}
