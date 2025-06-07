use crate::{
    ad,
    sol::{
        FitKey, ItemKey, ItemTypeId, SolarSystem,
        api::{FitMut, FwEffectMut},
        uad::item::{UadFwEffect, UadItem},
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_fw_effect(&mut self, fit_key: FitKey, a_item_id: ad::AItemId) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_fw_effect = UadFwEffect::new(&self.uad.src, item_id, a_item_id, fit_key, true);
        let uad_item = UadItem::FwEffect(uad_fw_effect);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.fw_effects.insert(item_key);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_item_without_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_fw_effect(&mut self, type_id: ItemTypeId) -> FwEffectMut<'_> {
        let item_key = self.sol.internal_add_fw_effect(self.key, type_id);
        FwEffectMut::new(self.sol, item_key)
    }
}
