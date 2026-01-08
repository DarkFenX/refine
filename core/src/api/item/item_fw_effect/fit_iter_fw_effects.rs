use crate::{
    api::{Fit, FitMut, FwEffect, FwEffectMut, MutIter},
    sol::SolarSystem,
    ud::UFitId,
};

impl<'a> Fit<'a> {
    pub fn iter_fw_effects(&self) -> impl ExactSizeIterator<Item = FwEffect<'_>> {
        iter_fw_effects(self.sol, self.uid)
    }
}

impl<'a> FitMut<'a> {
    pub fn iter_fw_effects(&self) -> impl ExactSizeIterator<Item = FwEffect<'_>> {
        iter_fw_effects(self.sol, self.uid)
    }
    pub fn iter_fw_effects_mut(&mut self) -> MutIter<'_, FwEffectMut<'_>> {
        let fw_effect_uids = self.sol.u_data.fits.get(self.uid).fw_effects.iter().copied().collect();
        MutIter::new(self.sol, fw_effect_uids)
    }
}

fn iter_fw_effects(sol: &SolarSystem, fit_uid: UFitId) -> impl ExactSizeIterator<Item = FwEffect<'_>> {
    let fw_effect_uids = sol.u_data.fits.get(fit_uid).fw_effects.iter();
    fw_effect_uids.map(|fw_effect_uid| FwEffect::new(sol, *fw_effect_uid))
}
