use crate::{SolarSystem, Subsystem, sol::UndoStep, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Subsystem<'s> {
    pub fn set_state(&mut self, state: bool) {
        let u_subsystem = self.sol.u_data.items.get(self.uid).dc_subsystem().unwrap();
        if state == u_subsystem.get_subsystem_state() {
            return;
        }
        self.sol.internal_set_subsystem_state(self.uid, state)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub(in crate::api) fn internal_set_subsystem_state(&mut self, subsystem_uid: UItemId, state: bool) {
        let u_subsystem = self.u_data.items.get_mut(subsystem_uid).dc_subsystem_mut().unwrap();
        if self.undo.is_recording() {
            self.undo.record(UndoStep::SubsystemState(
                subsystem_uid,
                u_subsystem.get_subsystem_state(),
            ));
        }
        let old_a_state = u_subsystem.get_state();
        u_subsystem.set_subsystem_state(state);
        let new_a_state = u_subsystem.get_state();
        u_subsystem.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            subsystem_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}
