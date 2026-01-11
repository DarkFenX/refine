use crate::{
    ad::AItemId,
    api::{BoosterMut, ItemTypeId},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_booster_type_aid(
        &mut self,
        booster_uid: UItemId,
        type_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(booster_uid);
        if u_item.get_type_aid() == type_aid {
            return;
        }
        SolarSystem::util_remove_booster(&mut self.u_data, &mut self.svc, booster_uid, reuse_eupdates);
        let u_booster = self.u_data.items.get_mut(booster_uid).dc_booster_mut().unwrap();
        u_booster.set_type_aid(type_aid, &self.u_data.src);
        SolarSystem::util_add_booster(&mut self.u_data, &mut self.svc, booster_uid, reuse_eupdates);
    }
}

impl<'a> BoosterMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_booster_type_aid(self.uid, type_id.into_aid(), &mut reuse_eupdates)
    }
}
