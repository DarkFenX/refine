use crate::sol::{
    FitKey, ModRack, SolarSystem,
    api::{Fit, FitMut, Module},
};

use super::shared::get_fit_rack;

impl<'a> Fit<'a> {
    pub fn iter_modules(&self, rack: ModRack) -> impl ExactSizeIterator<Item = Option<Module>> {
        iter_modules(self.sol, self.key, rack)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_modules(&self, rack: ModRack) -> impl ExactSizeIterator<Item = Option<Module>> {
        iter_modules(self.sol, self.key, rack)
    }
}

fn iter_modules(sol: &SolarSystem, fit_key: FitKey, rack: ModRack) -> impl ExactSizeIterator<Item = Option<Module>> {
    get_fit_rack(&sol.uad.fits, fit_key, rack)
        .iter_all()
        .map(|item_key_opt| item_key_opt.map(|item_key| Module::new(sol, item_key)))
}
