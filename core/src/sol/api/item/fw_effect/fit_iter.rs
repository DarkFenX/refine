use crate::sol::{
    FitKey, SolarSystem,
    api::{Fit, FitMut, FwEffect},
};

impl<'a> Fit<'a> {
    pub fn iter_fw_effects(&self) -> impl ExactSizeIterator<Item = FwEffect> {
        iter_fw_effects(self.sol, self.key)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_fw_effects(&self) -> impl ExactSizeIterator<Item = FwEffect> {
        iter_fw_effects(self.sol, self.key)
    }
}

fn iter_fw_effects(sol: &SolarSystem, fit_key: FitKey) -> impl ExactSizeIterator<Item = FwEffect> {
    sol.uad
        .fits
        .get(fit_key)
        .fw_effects
        .iter()
        .map(|item_key| FwEffect::new(sol, *item_key))
}
