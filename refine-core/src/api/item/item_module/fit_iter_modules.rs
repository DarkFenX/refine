use lender::{ExactSizeLender, Lender, Lending, check_covariance};

use crate::{
    api::{Fit, Module},
    misc::ModRack,
    sol::SolarSystem,
    ud::UItemId,
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
    type Lend = Option<Module<'lend>>;
}
impl<'iter> Lender for ModuleIter<'iter> {
    check_covariance!();

    fn next(&mut self) -> Option<Option<Module<'_>>> {
        let module_uid = *self.module_uids.get(self.index)?;
        self.index += 1;
        Some(module_uid.map(|module_uid| Module::new(self.sol, module_uid)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.module_uids.len() - self.index;
        (remaining, Some(remaining))
    }
}
impl<'iter> ExactSizeLender for ModuleIter<'iter> {}

impl<'s> Fit<'s> {
    pub fn iter_modules(&mut self, rack: ModRack) -> ModuleIter<'_> {
        let u_fit = self.sol.u_data.fits.get(self.uid);
        let u_module_vec = match rack {
            ModRack::High => &u_fit.mods_high,
            ModRack::Mid => &u_fit.mods_mid,
            ModRack::Low => &u_fit.mods_low,
        };
        let module_uids = u_module_vec.iter_all().collect();
        ModuleIter::new(self.sol, module_uids)
    }
}
