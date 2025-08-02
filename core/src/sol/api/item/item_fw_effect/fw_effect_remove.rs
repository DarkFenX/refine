use crate::{
    sol::{SolarSystem, api::FwEffectMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_fw_effect(
        &mut self,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_fw_effect(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        let u_fw_effect = self.u_data.items.get(item_key).get_fw_effect().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_fw_effect.get_fit_key());
        u_fit.fw_effects.remove(&item_key);
        self.u_data.items.remove(item_key);
    }
}

impl<'a> FwEffectMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_fw_effect(self.key, &mut reuse_eupdates);
    }
}
