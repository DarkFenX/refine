use crate::{
    sol::{
        SolarSystem,
        api::{Fit, FitMut, FwEffect, FwEffectMut, MutIter},
    },
    ud::UFitKey,
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
        let implant_keys = self.sol.u_data.fits.get(self.key).fw_effects.iter().copied().collect();
        MutIter::new(self.sol, implant_keys)
    }
}

fn iter_fw_effects(sol: &SolarSystem, fit_key: UFitKey) -> impl ExactSizeIterator<Item = FwEffect<'_>> {
    let fw_effect_keys = sol.u_data.fits.get(fit_key).fw_effects.iter();
    fw_effect_keys.map(|fw_effect_key| FwEffect::new(sol, *fw_effect_key))
}
