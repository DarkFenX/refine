use crate::{
    ad::AItemId,
    def::ItemTypeId,
    sol::{SolarSystem, api::FighterMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fighter_type_id(
        &mut self,
        item_key: UItemKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(item_key);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_fighter_with_acs(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            item_key,
            reuse_eupdates,
        );
        let u_fighter = self.u_data.items.get_mut(item_key).get_fighter_mut().unwrap();
        u_fighter.set_type_id(type_id, &self.u_data.src);
        // Update just fighter, autocharges will copy updated projection ranges
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, item_key);
        SolarSystem::util_add_fighter_with_acs(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            item_key,
            reuse_eupdates,
        );
    }
}

impl<'a> FighterMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_fighter_type_id(self.key, type_id, &mut reuse_eupdates)
    }
}
