use crate::{
    ad::AItemId,
    api::{FighterMut, ItemTypeId},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fighter_type_id(
        &mut self,
        fighter_uid: UItemId,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(fighter_uid);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_fighter_with_acs(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_uid,
            reuse_eupdates,
        );
        let u_fighter = self.u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
        u_fighter.set_type_id(type_id, &self.u_data.src);
        // Update just fighter, autocharges will copy updated projection ranges
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, fighter_uid);
        SolarSystem::util_add_fighter_with_acs(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_uid,
            reuse_eupdates,
        );
    }
}

impl<'a> FighterMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_fighter_type_id(self.uid, type_id.into_aid(), &mut reuse_eupdates)
    }
}
