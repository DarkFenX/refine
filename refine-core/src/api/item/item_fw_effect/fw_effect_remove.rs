use crate::{api::FwEffectMut, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    pub(in crate::api::item) fn internal_remove_fw_effect(&mut self, fw_effect_uid: UItemId) {
        SolarSystem::util_remove_fw_effect(&mut self.u_data, &mut self.svc, fw_effect_uid, &mut self.cache.eupdates);
        let u_fw_effect = self.u_data.items.get(fw_effect_uid).dc_fw_effect().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_fw_effect.get_fit_uid());
        u_fit.fw_effects.remove(&fw_effect_uid);
        self.u_data.items.remove(fw_effect_uid);
    }
}

impl<'s> FwEffectMut<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_fw_effect(self.uid);
    }
}
