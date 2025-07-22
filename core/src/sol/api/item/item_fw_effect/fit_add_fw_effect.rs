use crate::{
    ad,
    def::ItemTypeId,
    sol::{
        SolarSystem,
        api::{FitMut, FwEffectMut},
    },
    uad::{UadEffectUpdates, UadFitKey, UadFwEffect, UadItem, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_fw_effect(
        &mut self,
        fit_key: UadFitKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> UadItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_fw_effect = UadFwEffect::new(item_id, a_item_id, fit_key, true, &self.uad.src, reuse_eupdates);
        let uad_item = UadItem::FwEffect(uad_fw_effect);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.fw_effects.insert(item_key);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_item_without_projs(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_fw_effect(&mut self, type_id: ItemTypeId) -> FwEffectMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self.sol.internal_add_fw_effect(self.key, type_id, &mut reuse_eupdates);
        FwEffectMut::new(self.sol, item_key)
    }
}
