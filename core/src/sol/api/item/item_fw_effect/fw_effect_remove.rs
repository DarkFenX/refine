use crate::{
    sol::{SolarSystem, api::FwEffectMut},
    uad::{UadEffectUpdates, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_fw_effect(
        &mut self,
        item_key: UadItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_item = self.uad.items.get(item_key);
        let uad_fw_effect = uad_item.get_fw_effect().unwrap();
        SolarSystem::util_remove_item_without_projs(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        let uad_fit = self.uad.fits.get_mut(uad_fw_effect.get_fit_key());
        uad_fit.fw_effects.remove(&item_key);
        self.uad.items.remove(item_key);
    }
}

impl<'a> FwEffectMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_remove_fw_effect(self.key, &mut reuse_eupdates);
    }
}
