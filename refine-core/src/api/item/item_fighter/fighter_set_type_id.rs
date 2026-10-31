use crate::{
    ad::AItemId,
    api::{FighterMut, ItemTypeId},
    sol::{SolarSystem, UndoStep},
    ud::{UAutocharges, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fighter_type_aid(&mut self, fighter_uid: UItemId, type_aid: AItemId) {
        let u_item = self.u_data.items.get(fighter_uid);
        let old_type_aid = u_item.get_base_type_aid();
        if old_type_aid == type_aid {
            return;
        }
        // Autocharges are defined by fighter type, so they are replaced. Old ones are discarded as
        // fighter's children, and while recording their container goes to the log, so that
        // rollback can put them back with their IDs and states
        SolarSystem::util_detach_item_autocharges(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_uid,
            &mut self.cache.eupdates,
        );
        let u_fighter = self.u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
        let old_autocharges = u_fighter.take_autocharges();
        for autocharge_uid in old_autocharges.values() {
            self.internal_discard_child_item(autocharge_uid);
        }
        SolarSystem::util_remove_fighter(&mut self.u_data, &mut self.svc, fighter_uid, &mut self.cache.eupdates);
        let u_fighter = self.u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
        u_fighter.set_type_aid(type_aid, &self.u_data.r_data);
        if self.undo.is_recording() {
            self.undo
                .record_autocharges(UndoStep::FighterType(fighter_uid, old_type_aid), old_autocharges);
        }
        // Update just fighter, autocharges will copy updated projection ranges
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, fighter_uid);
        SolarSystem::util_add_fighter_with_acs(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_uid,
            &mut self.cache.eupdates,
        );
    }
    /// Puts back fighter type replaced by a type change, along with autocharges of that type.
    pub(in crate::api) fn internal_restore_fighter_type_aid(
        &mut self,
        fighter_uid: UItemId,
        type_aid: AItemId,
        autocharges: UAutocharges,
    ) {
        SolarSystem::util_remove_fighter_with_acs(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_uid,
            &mut self.cache.eupdates,
        );
        for autocharge_uid in autocharges.values() {
            self.u_data.items.reattach(autocharge_uid);
        }
        let u_fighter = self.u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
        u_fighter.set_type_aid(type_aid, &self.u_data.r_data);
        u_fighter.set_autocharges(autocharges);
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, fighter_uid);
        SolarSystem::util_add_fighter(&mut self.u_data, &mut self.svc, fighter_uid, &mut self.cache.eupdates);
        SolarSystem::util_reattach_item_autocharges(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_uid,
            &mut self.cache.eupdates,
        );
    }
}

impl<'s> FighterMut<'s> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_fighter_type_aid(self.uid, type_id.into_aid())
    }
}
