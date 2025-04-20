use lender::{Lender, Lending};

use crate::sol::{
    FitKey, ItemKey, ModRack, SolarSystem,
    api::{Fit, FitMut, Module, ModuleMut},
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
    pub fn iter_modules_mut(&mut self, rack: ModRack) -> ModuleIter {
        let module_keys = get_fit_rack(&self.sol.uad.fits, self.key, rack)
            .iter_all()
            .copied()
            .collect();
        ModuleIter {
            sol: self.sol,
            module_keys,
            index: 0,
        }
    }
}

fn iter_modules(sol: &SolarSystem, fit_key: FitKey, rack: ModRack) -> impl ExactSizeIterator<Item = Option<Module>> {
    get_fit_rack(&sol.uad.fits, fit_key, rack)
        .iter_all()
        .map(|item_key_opt| item_key_opt.map(|item_key| Module::new(sol, item_key)))
}

pub struct ModuleIter<'iter> {
    sol: &'iter mut SolarSystem,
    module_keys: Vec<Option<ItemKey>>,
    index: usize,
}
impl<'iter, 'lend> Lending<'lend> for ModuleIter<'iter> {
    type Lend = Option<ModuleMut<'lend>>;
}
impl<'iter> Lender for ModuleIter<'iter> {
    fn next(&mut self) -> Option<Option<ModuleMut>> {
        let module_key = *self.module_keys.get(self.index)?;
        self.index += 1;
        Some(module_key.map(|module_key| ModuleMut::new(self.sol, module_key)))
    }
}
