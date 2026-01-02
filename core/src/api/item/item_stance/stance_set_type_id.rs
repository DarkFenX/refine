use crate::{
    ad::AItemId,
    api::StanceMut,
    def::ItemTypeId,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_stance_type_id(
        &mut self,
        stance_key: UItemId,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(stance_key);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_stance(&mut self.u_data, &mut self.svc, stance_key, reuse_eupdates);
        let u_stance = self.u_data.items.get_mut(stance_key).dc_stance_mut().unwrap();
        u_stance.set_type_id(type_id, &self.u_data.src);
        SolarSystem::util_add_stance(&mut self.u_data, &mut self.svc, stance_key, reuse_eupdates);
    }
}

impl<'a> StanceMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_stance_type_id(self.key, type_id, &mut reuse_eupdates)
    }
}
