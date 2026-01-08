use lender::{Lender, Lending};

use super::shared::get_fit_rack;
use crate::{
    api::{Fit, FitMut, Module, ModuleMut},
    misc::ModRack,
    sol::SolarSystem,
    ud::{UFitId, UItemId},
};

pub struct ModuleIter<'iter> {
    sol: &'iter mut SolarSystem,
    module_uids: Vec<Option<UItemId>>,
    index: usize,
}
impl<'iter> ModuleIter<'iter> {
    fn new(sol: &'iter mut SolarSystem, module_uids: Vec<Option<UItemId>>) -> Self {
        Self {
            sol,
            module_uids,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for ModuleIter<'iter> {
    type Lend = Option<ModuleMut<'lend>>;
}
impl<'iter> Lender for ModuleIter<'iter> {
    fn next(&mut self) -> Option<Option<ModuleMut<'_>>> {
        let module_uid = *self.module_uids.get(self.index)?;
        self.index += 1;
        Some(module_uid.map(|module_uid| ModuleMut::new(self.sol, module_uid)))
    }
}

impl<'a> Fit<'a> {
    pub fn iter_modules(&self, rack: ModRack) -> impl ExactSizeIterator<Item = Option<Module<'_>>> {
        iter_modules(self.sol, self.uid, rack)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_modules(&self, rack: ModRack) -> impl ExactSizeIterator<Item = Option<Module<'_>>> {
        iter_modules(self.sol, self.uid, rack)
    }
    pub fn iter_modules_mut(&mut self, rack: ModRack) -> ModuleIter<'_> {
        let u_module_vec = get_fit_rack(&self.sol.u_data.fits, self.uid, rack);
        let module_uids = u_module_vec.iter_all().copied().collect();
        ModuleIter::new(self.sol, module_uids)
    }
}

fn iter_modules(
    sol: &SolarSystem,
    fit_uid: UFitId,
    rack: ModRack,
) -> impl ExactSizeIterator<Item = Option<Module<'_>>> {
    let u_module_vec = get_fit_rack(&sol.u_data.fits, fit_uid, rack);
    u_module_vec
        .iter_all()
        .map(|module_uid| module_uid.map(|module_uid| Module::new(sol, module_uid)))
}
