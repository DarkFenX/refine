use lender::{Lender, Lending};

use super::shared::get_fit_rack;
use crate::{
    misc::ModRack,
    sol::{
        SolarSystem,
        api::{Fit, FitMut, Module, ModuleMut},
    },
    ud::{UFitKey, UItemKey},
};

pub struct ModuleIter<'iter> {
    sol: &'iter mut SolarSystem,
    module_keys: Vec<Option<UItemKey>>,
    index: usize,
}
impl<'iter> ModuleIter<'iter> {
    fn new(sol: &'iter mut SolarSystem, module_keys: Vec<Option<UItemKey>>) -> Self {
        Self {
            sol,
            module_keys,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for ModuleIter<'iter> {
    type Lend = Option<ModuleMut<'lend>>;
}
impl<'iter> Lender for ModuleIter<'iter> {
    fn next(&mut self) -> Option<Option<ModuleMut<'_>>> {
        let module_key = *self.module_keys.get(self.index)?;
        self.index += 1;
        Some(module_key.map(|module_key| ModuleMut::new(self.sol, module_key)))
    }
}

impl<'a> Fit<'a> {
    pub fn iter_modules(&self, rack: ModRack) -> impl ExactSizeIterator<Item = Option<Module<'_>>> {
        iter_modules(self.sol, self.key, rack)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_modules(&self, rack: ModRack) -> impl ExactSizeIterator<Item = Option<Module<'_>>> {
        iter_modules(self.sol, self.key, rack)
    }
    pub fn iter_modules_mut(&mut self, rack: ModRack) -> ModuleIter<'_> {
        let module_keys = get_fit_rack(&self.sol.u_data.fits, self.key, rack)
            .iter_all()
            .copied()
            .collect();
        ModuleIter::new(self.sol, module_keys)
    }
}

fn iter_modules(
    sol: &SolarSystem,
    fit_key: UFitKey,
    rack: ModRack,
) -> impl ExactSizeIterator<Item = Option<Module<'_>>> {
    get_fit_rack(&sol.u_data.fits, fit_key, rack)
        .iter_all()
        .map(|item_key_opt| item_key_opt.map(|item_key| Module::new(sol, item_key)))
}
