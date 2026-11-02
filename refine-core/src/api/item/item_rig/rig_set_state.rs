use crate::{
    api::RigMut,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_rig_state(&mut self, rig_uid: UItemId, state: bool) {
        let u_rig = self.u_data.items.get_mut(rig_uid).dc_rig_mut().unwrap();
        let old_state = u_rig.get_rig_state();
        if state == old_state {
            return;
        }
        if self.undo.is_recording() {
            self.undo.record(UndoStep::RigState(rig_uid, old_state));
        }
        let old_a_state = u_rig.get_state();
        u_rig.set_rig_state(state);
        let new_a_state = u_rig.get_state();
        u_rig.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            rig_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}

impl<'s> RigMut<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_rig_state(self.uid, state)
    }
}
