use crate::{
    ad::AItemId,
    api::{ItemTypeId, StanceMut},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_stance_type_id(
        &mut self,
        stance_uid: UItemId,
        item_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(stance_uid);
        if u_item.get_type_id() == item_aid {
            return;
        }
        SolarSystem::util_remove_stance(&mut self.u_data, &mut self.svc, stance_uid, reuse_eupdates);
        let u_stance = self.u_data.items.get_mut(stance_uid).dc_stance_mut().unwrap();
        u_stance.set_type_id(item_aid, &self.u_data.src);
        SolarSystem::util_add_stance(&mut self.u_data, &mut self.svc, stance_uid, reuse_eupdates);
    }
}

impl<'a> StanceMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_stance_type_id(self.uid, type_id.into_aid(), &mut reuse_eupdates)
    }
}
