use crate::{
    ad::AItemId,
    api::{FitMut, FwEffectMut, ItemTypeId},
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitId, UFwEffect, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_fw_effect(
        &mut self,
        fit_uid: UFitId,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let item_id = self.u_data.items.alloc_id();
        let u_fw_effect = UFwEffect::new(item_id, type_id, fit_uid, true, &self.u_data.src);
        let u_item = UItem::FwEffect(u_fw_effect);
        let fw_effect_uid = self.u_data.items.add(u_item);
        u_fit.fw_effects.insert(fw_effect_uid);
        SolarSystem::util_add_fw_effect(&mut self.u_data, &mut self.svc, fw_effect_uid, reuse_eupdates);
        fw_effect_uid
    }
}

impl<'a> FitMut<'a> {
    pub fn add_fw_effect(&mut self, type_id: ItemTypeId) -> FwEffectMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let fw_effect_uid = self
            .sol
            .internal_add_fw_effect(self.uid, type_id.into_aid(), &mut reuse_eupdates);
        FwEffectMut::new(self.sol, fw_effect_uid)
    }
}
