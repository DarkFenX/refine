use crate::{
    api::FwEffectMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_fw_effect(
        &mut self,
        fw_effect_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_fw_effect(&mut self.u_data, &mut self.svc, fw_effect_key, reuse_eupdates);
        let u_fw_effect = self.u_data.items.get(fw_effect_key).dc_fw_effect().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_fw_effect.get_fit_key());
        u_fit.fw_effects.remove(&fw_effect_key);
        self.u_data.items.remove(fw_effect_key);
    }
}

impl<'a> FwEffectMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_fw_effect(self.key, &mut reuse_eupdates);
    }
}
