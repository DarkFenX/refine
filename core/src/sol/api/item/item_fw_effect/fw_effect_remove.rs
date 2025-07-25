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
        let u_item = self.u_data.items.get(item_key);
        let u_fw_effect = u_item.get_fw_effect().unwrap();
        SolarSystem::util_remove_item_without_projs(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
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
