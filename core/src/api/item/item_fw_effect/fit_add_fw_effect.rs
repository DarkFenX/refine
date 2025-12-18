use crate::{
    ad::AItemId,
    api::{FitMut, FwEffectMut},
    def::ItemTypeId,
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitKey, UFwEffect, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_fw_effect(
        &mut self,
        fit_key: UFitKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get_mut(fit_key);
        let item_id = self.u_data.items.alloc_id();
        let u_fw_effect = UFwEffect::new(item_id, type_id, fit_key, true, &self.u_data.src);
        let u_item = UItem::FwEffect(u_fw_effect);
        let fw_effect_key = self.u_data.items.add(u_item);
        u_fit.fw_effects.insert(fw_effect_key);
        SolarSystem::util_add_fw_effect(&mut self.u_data, &mut self.svc, fw_effect_key, reuse_eupdates);
        fw_effect_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_fw_effect(&mut self, type_id: ItemTypeId) -> FwEffectMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let fw_effect_key = self.sol.internal_add_fw_effect(self.key, type_id, &mut reuse_eupdates);
        FwEffectMut::new(self.sol, fw_effect_key)
    }
}
