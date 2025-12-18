use crate::{
    ad::AItemId,
    api::FwEffectMut,
    def::ItemTypeId,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fw_effect_type_id(
        &mut self,
        fw_effect_key: UItemKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(fw_effect_key);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_fw_effect(&mut self.u_data, &mut self.svc, fw_effect_key, reuse_eupdates);
        let u_fw_effect = self.u_data.items.get_mut(fw_effect_key).dc_fw_effect_mut().unwrap();
        u_fw_effect.set_type_id(type_id, &self.u_data.src);
        SolarSystem::util_add_fw_effect(&mut self.u_data, &mut self.svc, fw_effect_key, reuse_eupdates);
    }
}

impl<'a> FwEffectMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_fw_effect_type_id(self.key, type_id, &mut reuse_eupdates)
    }
}
