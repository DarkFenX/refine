use itertools::Itertools;

use crate::{
    Fighter, MinionState,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fighter<'s> {
    pub fn set_state(&mut self, state: MinionState) {
        let u_fighter = self.sol.u_data.items.get(self.uid).dc_fighter().unwrap();
        if state == u_fighter.get_fighter_state() {
            return;
        }
        self.sol.internal_set_fighter_state(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_fighter_state(&mut self, fighter_uid: UItemId, state: MinionState) {
        // Update user data for fighter
        let u_fighter = self.u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::FighterState(fighter_uid, u_fighter.get_fighter_state()));
        }
        let old_state = u_fighter.get_state();
        u_fighter.set_fighter_state(state);
        let new_state = u_fighter.get_state();
        u_fighter.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        // Filter out autocharges which couldn't be added due to any reason (no attribute which
        // defines them in the data, or couldn't be loaded), and fill autocharge key data
        let ac_activations = self
            .reuse
            .eupdates
            .autocharges
            .iter()
            .filter_map(|ac_act| {
                u_fighter
                    .get_autocharges()
                    .get_ac_uid(&ac_act.effect_rid)
                    .map(|ac_uid| (ac_uid, ac_act.active))
            })
            .collect_vec();
        // Update services for fighter
        CoreInner::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            fighter_uid,
            old_state,
            new_state,
            &self.reuse.eupdates,
        );
        CoreInner::util_process_autocharge_activations(
            &mut self.u_data,
            &mut self.svc,
            ac_activations,
            &mut self.reuse.eupdates,
        );
    }
}
