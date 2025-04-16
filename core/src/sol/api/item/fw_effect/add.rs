use crate::sol::{
    FitKey, ItemKey, ItemTypeId, SolarSystem,
    api::{FitMut, FwEffectMut},
    uad::item::{UadFwEffect, UadItem},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_add_fw_effect(&mut self, fit_key: FitKey, type_id: ItemTypeId) -> ItemKey {
        let uad_fit = self.uad.fits.get_mut(fit_key);
        let item_id = self.uad.items.alloc_id();
        let uad_fw_effect = UadFwEffect::new(&self.uad.src, item_id, type_id, fit_key, true);
        let uad_item = UadItem::FwEffect(uad_fw_effect);
        let item_key = self.uad.items.add(uad_item);
        uad_fit.fw_effects.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn add_fw_effect(&'a mut self, type_id: ItemTypeId) -> FwEffectMut<'a> {
        let item_key = self.sol.internal_add_fw_effect(self.key, type_id);
        FwEffectMut::new(self.sol, item_key)
    }
}
