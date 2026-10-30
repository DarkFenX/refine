use crate::{
    ad::AItemId,
    api::{FitMut, FwEffectMut, ItemTypeId},
    sol::{SolarSystem, UndoStep},
    ud::{UFitId, UFwEffect, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_fw_effect(&mut self, fit_uid: UFitId, type_aid: AItemId) -> UItemId {
        let item_id = self.u_data.items.alloc_id();
        let u_fw_effect = UFwEffect::new(item_id, type_aid, fit_uid, true, &self.u_data.r_data);
        let u_item = UItem::FwEffect(u_fw_effect);
        let fw_effect_uid = self.u_data.items.add(u_item);
        self.attach_fw_effect(fit_uid, fw_effect_uid);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::FwEffectAdded(fw_effect_uid));
        }
        fw_effect_uid
    }
    pub(in crate::api) fn internal_reattach_fw_effect(&mut self, fw_effect_uid: UItemId) {
        self.u_data.items.reattach(fw_effect_uid);
        let u_fw_effect = self.u_data.items.get(fw_effect_uid).dc_fw_effect().unwrap();
        let fit_uid = u_fw_effect.get_fit_uid();
        self.attach_fw_effect(fit_uid, fw_effect_uid);
    }
    fn attach_fw_effect(&mut self, fit_uid: UFitId, fw_effect_uid: UItemId) {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.fw_effects.insert(fw_effect_uid);
        SolarSystem::util_add_fw_effect(&mut self.u_data, &mut self.svc, fw_effect_uid, &mut self.cache.eupdates);
    }
}

impl<'s> FitMut<'s> {
    pub fn add_fw_effect(&mut self, type_id: ItemTypeId) -> FwEffectMut<'_> {
        let fw_effect_uid = self.sol.internal_add_fw_effect(self.uid, type_id.into_aid());
        FwEffectMut::new(self.sol, fw_effect_uid)
    }
}
