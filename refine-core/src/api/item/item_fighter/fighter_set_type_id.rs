use itertools::Itertools;

use crate::{
    ad::AItemId,
    api::{FighterMut, ItemTypeId},
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fighter_type_aid(&mut self, fighter_uid: UItemId, type_aid: AItemId) {
        let u_item = self.u_data.items.get(fighter_uid);
        let old_type_aid = u_item.get_base_type_aid();
        if old_type_aid == type_aid {
            return;
        }
        match self.undo.is_recording() {
            // Autocharges of the old type are kept, so that rollback can put them back with their
            // IDs and states
            true => {
                SolarSystem::util_detach_item_autocharges(
                    &mut self.u_data,
                    &mut self.svc,
                    &mut self.rev_projs,
                    fighter_uid,
                    &mut self.cache.eupdates,
                );
                let u_fighter = self.u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
                let cont_acs = u_fighter.get_autocharges_mut();
                let autocharge_uids = cont_acs.values().collect_vec();
                cont_acs.clear();
                for autocharge_uid in autocharge_uids {
                    self.internal_discard_item(autocharge_uid, UndoStep::AutochargeRemoved);
                }
                SolarSystem::util_remove_fighter(
                    &mut self.u_data,
                    &mut self.svc,
                    fighter_uid,
                    &mut self.cache.eupdates,
                );
            }
            false => SolarSystem::util_remove_fighter_with_acs(
                &mut self.u_data,
                &mut self.svc,
                &mut self.rev_projs,
                fighter_uid,
                &mut self.cache.eupdates,
            ),
        }
        let u_fighter = self.u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
        u_fighter.set_type_aid(type_aid, &self.u_data.r_data);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::FighterType(fighter_uid, old_type_aid));
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
    /// Changes type without adding autocharges: autocharges of the restored type are put back by
    /// their own steps, which are undone right after this one.
    pub(in crate::api) fn internal_restore_fighter_type_aid(&mut self, fighter_uid: UItemId, type_aid: AItemId) {
        SolarSystem::util_remove_fighter_with_acs(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_uid,
            &mut self.cache.eupdates,
        );
        let u_fighter = self.u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
        u_fighter.set_type_aid(type_aid, &self.u_data.r_data);
        SolarSystem::util_update_item_radius_in_projs(&mut self.u_data, &self.rev_projs, &mut self.svc, fighter_uid);
        SolarSystem::util_add_fighter(&mut self.u_data, &mut self.svc, fighter_uid, &mut self.cache.eupdates);
    }
}

impl<'s> FighterMut<'s> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_fighter_type_aid(self.uid, type_id.into_aid())
    }
}
